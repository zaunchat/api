use axum::response::Redirect;

pub async fn login() -> Redirect {
    Redirect::permanent("/sessions")
}
