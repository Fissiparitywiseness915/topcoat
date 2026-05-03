use topcoat::{
    context::Cx,
    router::{RedirectExt, Result, page, path_param},
    view::{View, view},
};

#[path_param]
struct PostId(uuid::Uuid);

#[page]
async fn post_page(cx: &Cx) -> Result {
    let post_id = PostId::of(cx)
        .as_ref()
        .unwrap_or_redirect(cx, "/what")
        .await;
    view! { "showing post with id: " (post_id.to_string()) }
}
