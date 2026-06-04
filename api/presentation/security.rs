use worker::Response;

pub fn apply_security_headers(mut resp: Response) -> Response {
    let headers = resp.headers_mut();
    let _ = headers.set("X-Content-Type-Options", "nosniff");
    let _ = headers.set("X-Frame-Options", "DENY");
    let _ = headers.set("X-XSS-Protection", "1; mode=block");
    let _ = headers.set(
        "Strict-Transport-Security",
        "max-age=31536000; includeSubDomains",
    );
    let _ = headers.set("Referrer-Policy", "strict-origin-when-cross-origin");
    let _ = headers.set(
        "Permissions-Policy",
        "geolocation=(), microphone=(), camera=()",
    );
    if headers
        .get("Content-Security-Policy")
        .ok()
        .flatten()
        .is_none()
    {
        let _ = headers.set(
            "Content-Security-Policy",
            "default-src 'none'; frame-ancestors 'none'",
        );
    }
    resp
}
