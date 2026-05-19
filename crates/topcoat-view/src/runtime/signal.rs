use std::iter::empty;

use serde::{Deserialize, Serialize};
use topcoat_core::context::Cx;
use uuid::Uuid;

use crate::runtime::{IntoViewParts, Island, Unescaped, View, ViewPart};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SignalId(Uuid);

impl SignalId {
    #[inline]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for SignalId {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Signal<T> {
    id: SignalId,
    value: T,
}

impl<T> Signal<T> {
    #[inline]
    pub fn new(value: T) -> Self {
        Self {
            id: SignalId::new(),
            value,
        }
    }
}

pub struct SignalDeclaration<'a, T>(&'a Signal<T>);

impl<'a, T> SignalDeclaration<'a, T> {
    #[inline]
    pub fn new(signal: &'a Signal<T>) -> Self {
        Self(signal)
    }
}

impl<T> IntoViewParts for SignalDeclaration<'_, T>
where
    T: Serialize,
{
    fn into_view_parts(self) -> impl Iterator<Item = super::ViewPart> {
        [
            ViewPart::UnescapedStaticStr(Unescaped::new_unchecked("<!-- signal: ")),
            ViewPart::UnescapedString(Unescaped::new_unchecked(
                serde_json::to_string(&self.0).unwrap(),
            )),
            ViewPart::UnescapedStaticStr(Unescaped::new_unchecked(" -->")),
        ]
        .into_iter()
    }
}

#[derive(Debug, Deserialize)]
#[serde(bound(deserialize = "&'a T: Deserialize<'de>"))]
pub struct ReadSignal<'a, T: ?Sized> {
    id: SignalId,
    #[serde(borrow)]
    value: &'a T,
}

impl<'a, T> ReadSignal<'a, T> {
    pub fn new(signal: &'a Signal<T>) -> Self {
        Self {
            id: signal.id,
            value: &signal.value,
        }
    }
}

pub trait Signals<'de>: Sized {
    fn ids(&self) -> impl Iterator<Item = SignalId>;
    fn decode(encoded_signals: &'de EncodedSignals) -> Self;
}

impl<'de> Signals<'de> for () {
    fn ids(&self) -> impl Iterator<Item = SignalId> {
        empty()
    }

    fn decode(_encoded_signals: &'de EncodedSignals) -> Self {}
}

macro_rules! impl_signals_for_tuple {
    ($($n:tt $t:ident),+) => {
        impl<'de, $($t),+> Signals<'de> for ($(ReadSignal<'de, $t>,)+)
        where
            $($t: ?Sized + 'de,)+
            $(&'de $t: Deserialize<'de>,)+
        {
            fn ids(&self) -> impl Iterator<Item = SignalId> {
                [$(self.$n.id),+].into_iter()
            }

            fn decode(encoded_signals: &'de EncodedSignals) -> Self {
                serde_json::from_str(&encoded_signals.0).unwrap()
            }
        }
    };
}

impl_signals_for_tuple!(0 T0);
impl_signals_for_tuple!(0 T0, 1 T1);
impl_signals_for_tuple!(0 T0, 1 T1, 2 T2);
impl_signals_for_tuple!(0 T0, 1 T1, 2 T2, 3 T3);
impl_signals_for_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4);
impl_signals_for_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5);
impl_signals_for_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6);
impl_signals_for_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7);
impl_signals_for_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8);
impl_signals_for_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9);
impl_signals_for_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9, 10 T10);
impl_signals_for_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9, 10 T10, 11 T11);

pub struct EncodedSignals(String);

impl EncodedSignals {
    pub fn new(inner: impl Into<String>) -> Self {
        Self(inner.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct ReactiveScopeId(Uuid);

impl ReactiveScopeId {
    #[inline]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ReactiveScopeId {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

pub struct ReactiveScope {
    id: ReactiveScopeId,
    track: Vec<SignalId>,
    placeholder: View,
}

impl ReactiveScope {
    #[inline]
    pub async fn new<'de, S, E>(cx: &Cx, signals: &S, island: Island<S, E>) -> Result<Self, E>
    where
        S: Signals<'de>,
    {
        Ok(Self {
            id: ReactiveScopeId::new(),
            track: signals.ids().collect(),
            placeholder: island.render(cx, signals).await?,
        })
    }
}

impl IntoViewParts for ReactiveScope {
    fn into_view_parts(self) -> impl Iterator<Item = ViewPart> {
        [
            ViewPart::UnescapedStaticStr(Unescaped::new_unchecked("<!-- reactive scope start: ")),
            ViewPart::UnescapedString(Unescaped::new_unchecked(
                serde_json::to_string(&self.id).unwrap(),
            )),
            ViewPart::UnescapedStaticStr(Unescaped::new_unchecked(" ")),
            ViewPart::UnescapedString(Unescaped::new_unchecked(
                serde_json::to_string(&self.track).unwrap(),
            )),
            ViewPart::UnescapedStaticStr(Unescaped::new_unchecked(" -->")),
            self.placeholder.into_inner(),
            ViewPart::UnescapedStaticStr(Unescaped::new_unchecked("<!-- reactive scope end: ")),
            ViewPart::UnescapedString(Unescaped::new_unchecked(
                serde_json::to_string(&self.id).unwrap(),
            )),
            ViewPart::UnescapedStaticStr(Unescaped::new_unchecked(" -->")),
        ]
        .into_iter()
    }
}
