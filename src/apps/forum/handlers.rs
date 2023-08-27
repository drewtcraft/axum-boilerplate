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
) -> Result<impl IntoResponse> {
    let rendered = NewThreadTemplate::new_render(context.get_is_htmx())?;
    context.page_title = Some(String::from("Threads"));

    Ok(Html(rendered))
}

pub async fn post_create_thread(
    Extension(context): Extension<Context>,
    State(state): State<Arc<AppState>>,
    Form(body): Form<NewThread>,
) -> Result<impl IntoResponse> {
    let (valid, errors) = body.validate();
    if !valid {
        info!("new thread is invalid: {:?}", errors);
        let rendered = NewThreadTemplate::new_render_error(
            context.get_is_htmx(),
            body,
            errors,
        )?;

        return Ok(Html(rendered).into_response());
    }
    let user_id = context.user_data.unwrap().user_id;

    // TODO what if this took the same serializer as the template??
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
