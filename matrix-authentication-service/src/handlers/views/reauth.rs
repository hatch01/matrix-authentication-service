// Copyright 2021 The Matrix.org Foundation C.I.C.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use serde::Deserialize;
use sqlx::{pool::PoolConnection, PgPool, Postgres};
use warp::{hyper::Uri, reply::html, wrap_fn, Filter, Rejection, Reply};

use crate::{
    config::{CookiesConfig, CsrfConfig},
    errors::WrapError,
    filters::{
        csrf::{protected_form, save_csrf_token, updated_csrf_token},
        database::with_connection,
        session::with_session,
        with_templates, CsrfToken,
    },
    storage::SessionInfo,
    templates::{TemplateContext, Templates},
};

#[derive(Deserialize, Debug)]
struct ReauthForm {
    password: String,
}

pub(super) fn filter(
    pool: &PgPool,
    templates: &Templates,
    csrf_config: &CsrfConfig,
    cookies_config: &CookiesConfig,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone + Send + Sync + 'static {
    let get = warp::get()
        .and(with_templates(templates))
        .and(updated_csrf_token(cookies_config, csrf_config))
        .and(with_session(pool, cookies_config))
        .and_then(get)
        .untuple_one()
        .with(wrap_fn(save_csrf_token(cookies_config)));

    let post = warp::post()
        .and(with_session(pool, cookies_config))
        .and(with_connection(pool))
        .and(protected_form(cookies_config))
        .and_then(post);

    warp::path!("reauth").and(get.or(post))
}

async fn get(
    templates: Templates,
    csrf_token: CsrfToken,
    session: SessionInfo,
) -> Result<(CsrfToken, impl Reply), Rejection> {
    let ctx = ().with_session(session).with_csrf(&csrf_token);

    let content = templates.render_reauth(&ctx)?;
    Ok((csrf_token, html(content)))
}

async fn post(
    session: SessionInfo,
    mut conn: PoolConnection<Postgres>,
    form: ReauthForm,
) -> Result<impl Reply, Rejection> {
    let _session = session
        .reauth(&mut conn, &form.password)
        .await
        .wrap_error()?;

    Ok(warp::redirect(Uri::from_static("/")))
}
