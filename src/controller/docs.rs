use anyhow::Context;
use axum::{
    Json,
    response::{Html, IntoResponse},
};
use tracing::instrument;

use crate::Error;

const SWAGGER_UI_VERSION: &str = "5";

#[instrument(skip_all)]
pub async fn openapi() -> Result<impl IntoResponse, Error> {
    let definition: serde_json::Value =
        serde_json::from_str(include_str!("../../docs/openapi.json"))
            .context("failed to parse OpenAPI definition")?;
    Ok(Json(definition))
}

#[instrument(skip_all)]
pub async fn swagger() -> impl IntoResponse {
    Html(format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>API Documentation</title>
  <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@{SWAGGER_UI_VERSION}/swagger-ui.css" />
</head>
<body>
  <div id="swagger-ui"></div>
  <script src="https://unpkg.com/swagger-ui-dist@{SWAGGER_UI_VERSION}/swagger-ui-bundle.js" crossorigin></script>
  <script>
    window.onload = () => {{
      window.ui = SwaggerUIBundle({{ url: '/docs/openapi.json', dom_id: '#swagger-ui' }});
    }};
  </script>
</body>
</html>"#
    ))
}
