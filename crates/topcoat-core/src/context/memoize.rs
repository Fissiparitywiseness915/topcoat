//! Per-request memoization for expensive computations.
//!
//! Functions annotated with `#[memoize]` are evaluated at most once per set of arguments
//! within the same request context. Repeated calls with equal arguments return the cached
//! result instead of recomputing it.

use std::{
    any::{Any, TypeId},
    collections::hash_map::RandomState,
    future::Future,
    hash::Hash,
    marker::PhantomData,
    ops::Deref,
    sync::{Arc, Mutex, OnceLock},
};

use hashbrown::{Equivalent, HashMap};
use tokio::sync::OnceCell;

/// A handle to a memoized value, scoped to the request context.
///
/// `Memoized<T>` is returned by functions annotated with `#[memoize]`. It dereferences to
/// the underlying value, so it can be used wherever a `&T` is expected. The handle is tied
/// to the lifetime of the request context and cannot outlive it.
///
/// # Example
///
/// ```rust,ignore
/// #[memoize]
/// fn add(cx: &Cx, x: i32, y: i32) -> i32 {
///     println!("adding {x} + {y}");
///     x + y
/// }
///
/// async fn handler(cx: &Cx) {
///     // Prints "adding 5 + 6" once.
///     let a = add(cx, 5, 6);
///     // Returns the cached result without printing.
///     let b = add(cx, 5, 6);
///     // Different arguments compute a fresh value.
///     let c = add(cx, 5, 7);
///
///     assert_eq!(*a, 11);
///     assert_eq!(*b, 11);
///     assert_eq!(*c, 12);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Memoized<'a, T> {
    inner: Arc<T>,
    // We artificially limit the lifetime of a memoized value to be the lifetime of the request
    // context. This is because the `Arc` is an implementation detail of the cache. The user should
    // not be able to hold on to the memoized value as long as they want. Conceptually, the cache
    // only lasts as long as the request context. The implementation might change to be more
    // efficient in the future.
    lifetime: PhantomData<&'a ()>,
}

impl<'a, T> Memoized<'a, T> {
    fn new(inner: Arc<T>) -> Self {
        Self {
            inner,
            lifetime: PhantomData,
        }
    }
}

impl<'a, T> Deref for Memoized<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// Two-level cache: the outer map has one entry per memoized function (keyed by a `TypeId`
/// derived from the function's closure type), and each inner map (boxed as `dyn Any`) maps
/// that function's argument tuple to its cached cell.
#[doc(hidden)]
pub struct MemoizeCache {
    entries: Mutex<HashMap<TypeId, Box<dyn Any + Send + Sync>>>,
}

impl MemoizeCache {
    pub(super) fn new() -> Self {
        Self {
            entries: Mutex::new(HashMap::new()),
        }
    }

    pub fn memoize<'a, K, P, V, F>(&'a self, key: K, params: P, f: F) -> Memoized<'a, V>
    where
        K: Copy,
        MemoizeKey<K>: Hash + ToOwnedKey + Equivalent<<MemoizeKey<K> as ToOwnedKey>::Owned>,
        <MemoizeKey<K> as ToOwnedKey>::Owned: Hash + Eq + Send + Sync + 'static,
        V: Send + Sync + 'static,
        F: (FnOnce(P) -> V) + 'static,
    {
        let cell = self.cell_for::<F, _, OnceLock<Arc<V>>>(key);
        let value = cell.get_or_init(|| Arc::new(f(params)));
        Memoized::new(value.clone())
    }

    pub async fn memoize_async<'a, K, P, V, F, Fut>(
        &'a self,
        key: K,
        params: P,
        f: F,
    ) -> Memoized<'a, V>
    where
        K: Copy,
        MemoizeKey<K>: Hash + ToOwnedKey + Equivalent<<MemoizeKey<K> as ToOwnedKey>::Owned>,
        <MemoizeKey<K> as ToOwnedKey>::Owned: Hash + Eq + Send + Sync + 'static,
        V: Send + Sync + 'static,
        F: (FnOnce(P) -> Fut) + 'static,
        Fut: Future<Output = V>,
    {
        let cell = self.cell_for::<F, _, OnceCell<Arc<V>>>(key);
        let value = cell
            .get_or_init(|| async { Arc::new(f(params).await) })
            .await;
        Memoized::new(value.clone())
    }

    /// Returns the cell that holds the cached value for the given argument key. `Marker` is the
    /// closure type of the memoized function, used as a unique `TypeId` to pick the right inner
    /// map. The cell is wrapped in `Arc` so the caller can drop the outer lock before running
    /// (potentially expensive or async) initialization.
    fn cell_for<Marker, K, Cell>(&self, key: K) -> Arc<Cell>
    where
        Marker: 'static,
        K: Copy,
        MemoizeKey<K>: Hash + ToOwnedKey + Equivalent<<MemoizeKey<K> as ToOwnedKey>::Owned>,
        <MemoizeKey<K> as ToOwnedKey>::Owned: Hash + Eq + Send + Sync + 'static,
        Cell: Default + Send + Sync + 'static,
    {
        let mut guard = self.entries.lock().unwrap();
        let cache = guard.entry(TypeId::of::<Marker>()).or_insert_with(|| {
            Box::new(HashMap::<
                <MemoizeKey<K> as ToOwnedKey>::Owned,
                Arc<Cell>,
                RandomState,
            >::with_hasher(RandomState::new()))
        });
        let cache = cache
            .downcast_mut::<HashMap<<MemoizeKey<K> as ToOwnedKey>::Owned, Arc<Cell>, RandomState>>()
            .unwrap();

        // Look up using the borrowed key via `Equivalent` to avoid cloning the arguments on
        // cache hits; only clone into an owned key when inserting.
        if let Some(cell) = cache.get(&MemoizeKey(key)) {
            cell.clone()
        } else {
            let cell = Arc::new(Cell::default());
            let key_owned = MemoizeKey(key).to_owned_key();
            cache.insert(key_owned, cell.clone());
            cell
        }
    }
}

impl std::fmt::Debug for MemoizeCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemoizeCache").finish()
    }
}

/// A newtype wrapper around the argument tuple. It exists so we can implement `Equivalent` and
/// `ToOwnedKey` for tuples of references against the corresponding tuple of owned values, which
/// would otherwise run into orphan rules and conflicting blanket impls.
#[doc(hidden)]
#[derive(Hash)]
pub struct MemoizeKey<T>(T);

/// Converts a borrowed key (e.g. `(&str, &i32)`) into the owned key stored in the map
/// (e.g. `(String, i32)`). Used only on cache misses, when we need to insert.
#[doc(hidden)]
pub trait ToOwnedKey {
    type Owned;
    fn to_owned_key(&self) -> Self::Owned;
}

/// Generates `Equivalent` and `ToOwnedKey` impls for argument tuples up to arity 12, so callers
/// can pass keys made of borrowed values and still hit entries stored as owned values.
macro_rules! impl_tuple {
    ($(($kty:ident, $qty:ident, $accessor:tt)),*) => {
        impl<$($kty, $qty),*> Equivalent<($($kty,)*)> for MemoizeKey<($(&$qty,)*)>
        where
            $(
                $qty: ?Sized + Equivalent<$kty>,
            )*
        {
            fn equivalent(&self, key: &($($kty,)*)) -> bool {
                $(self.0.$accessor.equivalent(&key.$accessor))&&*
            }
        }

        impl<$($qty),*> ToOwnedKey for MemoizeKey<($(&$qty,)*)>
        where
            $($qty: ?Sized + ToOwned,)*
        {
            type Owned = ($($qty::Owned,)*);
            fn to_owned_key(&self) -> Self::Owned {
                ($(self.0.$accessor.to_owned(),)*)
            }
        }
    };
}

#[rustfmt::skip]
mod impls {
    use super::{Equivalent, MemoizeKey, ToOwnedKey};

    // Hand-written zero-arity impls for memoized functions whose only parameter is `cx`. The
    // macro's `&&*`-joined body doesn't expand cleanly for zero repetitions.
    impl Equivalent<()> for MemoizeKey<()> {
        fn equivalent(&self, _key: &()) -> bool { true }
    }
    impl ToOwnedKey for MemoizeKey<()> {
        type Owned = ();
        fn to_owned_key(&self) -> Self::Owned {}
    }

    impl_tuple!((K1, Q1, 0));
    impl_tuple!((K1, Q1, 0), (K2, Q2, 1));
    impl_tuple!((K1, Q1, 0), (K2, Q2, 1), (K3, Q3, 2));
    impl_tuple!((K1, Q1, 0), (K2, Q2, 1), (K3, Q3, 2), (K4, Q4, 3));
    impl_tuple!((K1, Q1, 0), (K2, Q2, 1), (K3, Q3, 2), (K4, Q4, 3), (K5, Q5, 4));
    impl_tuple!((K1, Q1, 0), (K2, Q2, 1), (K3, Q3, 2), (K4, Q4, 3), (K5, Q5, 4), (K6, Q6, 5));
    impl_tuple!((K1, Q1, 0), (K2, Q2, 1), (K3, Q3, 2), (K4, Q4, 3), (K5, Q5, 4), (K6, Q6, 5), (K7, Q7, 6));
    impl_tuple!((K1, Q1, 0), (K2, Q2, 1), (K3, Q3, 2), (K4, Q4, 3), (K5, Q5, 4), (K6, Q6, 5), (K7, Q7, 6), (K8, Q8, 7));
    impl_tuple!((K1, Q1, 0), (K2, Q2, 1), (K3, Q3, 2), (K4, Q4, 3), (K5, Q5, 4), (K6, Q6, 5), (K7, Q7, 6), (K8, Q8, 7), (K9, Q9, 8));
    impl_tuple!((K1, Q1, 0), (K2, Q2, 1), (K3, Q3, 2), (K4, Q4, 3), (K5, Q5, 4), (K6, Q6, 5), (K7, Q7, 6), (K8, Q8, 7), (K9, Q9, 8), (K10, Q10, 9));
    impl_tuple!((K1, Q1, 0), (K2, Q2, 1), (K3, Q3, 2), (K4, Q4, 3), (K5, Q5, 4), (K6, Q6, 5), (K7, Q7, 6), (K8, Q8, 7), (K9, Q9, 8), (K10, Q10, 9), (K11, Q11, 10));
    impl_tuple!((K1, Q1, 0), (K2, Q2, 1), (K3, Q3, 2), (K4, Q4, 3), (K5, Q5, 4), (K6, Q6, 5), (K7, Q7, 6), (K8, Q8, 7), (K9, Q9, 8), (K10, Q10, 9), (K11, Q11, 10), (K12, Q12, 11));
}
