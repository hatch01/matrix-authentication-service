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

use sqlx::PgPool;
use url::Url;
use warp::{reply::html, wrap_fn, Filter, Rejection, Reply};

use crate::{
    config::{CookiesConfig, CsrfConfig, OAuth2Config},
    filters::{
        csrf::{save_csrf_token, updated_csrf_token},
        session::with_optional_session,
        with_templates, CsrfToken,
    },
    storage::SessionInfo,
    templates::{IndexContext, TemplateContext, Templates},
};

pub(super) fn filter(
    pool: &PgPool,
    templates: &Templates,
    oauth2_config: &OAuth2Config,
    csrf_config: &CsrfConfig,
    cookies_config: &CookiesConfig,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone + Send + Sync + 'static {
    let discovery_url = oauth2_config.discovery_url();
    warp::path::end()
        .and(warp::get())
        .map(move || discovery_url.clone())
        .and(with_templates(templates))
        .and(updated_csrf_token(cookies_config, csrf_config))
        .and(with_optional_session(pool, cookies_config))
        .and_then(get)
        .untuple_one()
        .with(wrap_fn(save_csrf_token(cookies_config)))
}

async fn get(
    discovery_url: Url,
    templates: Templates,
    csrf_token: CsrfToken,
    session: Option<SessionInfo>,
) -> Result<(CsrfToken, impl Reply), Rejection> {
    let ctx = IndexContext::new(discovery_url)
        .maybe_with_session(session)
        .with_csrf(&csrf_token);

    let content = templates.render_index(&ctx)?;
    Ok((csrf_token, html(content)))
}
