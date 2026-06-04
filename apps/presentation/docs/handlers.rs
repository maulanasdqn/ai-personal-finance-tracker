use super::spec::OPENAPI_SPEC;
use worker::{Headers, Request, Response, RouteContext};

pub fn spec_handler(_req: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    let mut headers = Headers::new();
    headers.set("content-type", "application/json")?;
    headers.set("access-control-allow-origin", "*")?;
    let resp = Response::ok(OPENAPI_SPEC)?;
    Ok(resp.with_headers(headers))
}

pub fn ui_handler(_req: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    let html = r##"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>AI Finance Tracker — API Docs</title>
  <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5/swagger-ui.css" />
  <style>
    body { margin: 0; }
    .topbar { display: none; }
  </style>
</head>
<body>
  <div id="swagger-ui"></div>
  <script src="https://unpkg.com/swagger-ui-dist@5/swagger-ui-bundle.js"></script>
  <script>
    SwaggerUIBundle({
      url: "/api/docs/openapi.json",
      dom_id: "#swagger-ui",
      presets: [SwaggerUIBundle.presets.apis, SwaggerUIBundle.SwaggerUIStandalonePreset],
      layout: "BaseLayout",
      deepLinking: true,
      tryItOutEnabled: true,
      persistAuthorization: true,
    });
  </script>
</body>
</html>"##;

    let mut headers = Headers::new();
    headers.set("content-type", "text/html; charset=utf-8")?;
    headers.set(
        "Content-Security-Policy",
        "default-src 'none'; script-src 'unsafe-inline' https://unpkg.com; style-src 'unsafe-inline' https://unpkg.com; img-src 'self' data:; connect-src 'self'; font-src https://unpkg.com; frame-ancestors 'none'",
    )?;
    let resp = Response::ok(html)?;
    Ok(resp.with_headers(headers))
}
