use std::{marker::PhantomData, path::PathBuf};

// use topcoat::asset::AssetBundle;
//
// mod app;
// mod components;

struct Kek<R, T, F> {
    v: R,
    f: F,
    _phantom: PhantomData<T>,
}

impl<R, T, F> Kek<R, T, F>
where
    F: FnOnce(R) -> T,
{
    fn new(v: R, f: F) -> Self {
        Self {
            v,
            f,
            _phantom: PhantomData,
        }
    }
}

#[tokio::main]
async fn main() {
    let smep = Kek::<_, _, _>::new(5, {
        fn lel<T: std::ops::Add<i32>>(x: T) -> T::Output {
            x + 5
        }
        lel
    });
    let lel = (smep.f)(5);
    println!("{lel}");

    // let router = app::router()
    //     .assets(AssetBundle::load_dir(PathBuf::from("../../target/assets")).unwrap())
    //     .app_state(5);
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // topcoat::serve(listener, router).await.unwrap();
}
