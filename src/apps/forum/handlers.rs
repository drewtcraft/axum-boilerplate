use std::sync::Arc;

use axum::{
    extract::{Form, State, Path},
    response::{Html, IntoResponse, Redirect, Response},
    Extension,
};
use log::info;

use crate::{context::Context, error::Result, state::AppState, traits::ParamValidator, utils};

use super::{
    models::PostModel,
    serializers::{NewThread, PathId},
    templates::NewThreadTemplate,
};

pub async fn get_create_thread(
    Extension(mut context): Extension<Context>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    let rendered_new_thread = NewThreadTemplate::new_render()?;

    context.page_title = Some(String::from("Threads"));

    let html = utils::render_template(context.is_htmx, rendered_new_thread)?;

    Ok(Html(html))
}

pub async fn post_create_thread(
    Extension(context): Extension<Context>,
    State(state): State<Arc<AppState>>,
    Form(body): Form<NewThread>,
) -> Result<impl IntoResponse> {
    let (valid, errors) = body.validate();
    if !valid {
        info!("new thread is invalid: {:?}", errors);
        let rendered_new_thread = NewThreadTemplate::new_render_error(
            Some(body.title.as_str()),
            errors.title.as_ref().map(|s| s.as_str()),
            Some(body.content.as_str()),
            errors.text_content.as_ref().map(|s| s.as_str()),
        )?;

        let html = utils::render_template(context.is_htmx, rendered_new_thread)?;

        return Ok(Html(html).into_response());
    }
    let user_id = context.user_data.unwrap().user_id;

    let thread_id = PostModel::create_post(
        &state.db_pool,
        user_id,
        None,
        &body.title,
        &body.content,
    )
    .await?;

    info!("Created new thread {}", thread_id);

    let redirect = format!("/threads/{}", thread_id);

    Ok(Redirect::to(&redirect).into_response())
}

pub async fn get_thread(
    Extension(context): Extension<Context>,
    State(state): State<Arc<AppState>>,
    Path(id): Path<PathId>,
) -> Result<impl IntoResponse> {
    
    Ok(Html(""))

}

pub async fn get_threads(
    Extension(context): Extension<Context>,
    State(state): State<Arc<AppState>>,
    Form(body): Form<NewThread>,
) -> Result<impl IntoResponse> {
    Ok(Html(""))
}
