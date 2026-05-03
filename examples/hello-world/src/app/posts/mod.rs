use serde::Deserialize;
use topcoat::{
    context::Cx,
    router::{QueryParams, page},
    view::{View, view},
};

mod id;

#[derive(Deserialize, QueryParams)]
struct PageQuery {
    page: Option<u32>,
}

#[page]
async fn posts(cx: &Cx) -> View {
    view! {
        <div>"currently on page: " (PageQuery::of(cx).as_ref().unwrap().page)</div>
    }
}
