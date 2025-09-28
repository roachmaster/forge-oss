// crates/forge-web-ui/src/http.rs

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};           // <-- add Deserialize
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Headers, Request, RequestInit, RequestMode, Response};

/// Generic JSON POST helper for browser/WASM.
///
/// Usage:
///   let resp: MyResp = post_json("/v1/open", &my_req).await?;
pub async fn post_json<TReq, TResp>(path: &str, body: &TReq) -> Result<TResp, String>
where
    TReq: Serialize,
    TResp: DeserializeOwned,
{
    let win = window().ok_or_else(|| "window() not available".to_string())?;

    // Serialize the request body
    let json = serde_json::to_string(body).map_err(|e| format!("serialize error: {e}"))?;

    // Build request init
    let mut init = RequestInit::new();
    init.method("POST");
    init.mode(RequestMode::SameOrigin);

    // Headers
    let headers = Headers::new().map_err(|e| format!("headers error: {:?}", e))?;
    headers
        .set("Content-Type", "application/json")
        .map_err(|e| format!("set header error: {:?}", e))?;
    init.headers(&headers);
    init.body(Some(&wasm_bindgen::JsValue::from_str(&json)));

    // Build Request
    let req = Request::new_with_str_and_init(path, &init)
        .map_err(|e| format!("request init error: {:?}", e))?;

    // Fetch
    let resp_val = JsFuture::from(win.fetch_with_request(&req))
        .await
        .map_err(|e| format!("fetch error: {:?}", e))?;

    let resp: Response = resp_val.dyn_into().map_err(|_| "bad response".to_string())?;

    // Read response text
    let status = resp.status();
    let ok = resp.ok();
    let text_js = JsFuture::from(resp.text().map_err(|e| format!("resp.text() error: {:?}", e))?)
        .await
        .map_err(|e| format!("read body error: {:?}", e))?;
    let text = text_js.as_string().unwrap_or_default();

    if !ok {
        return Err(format!("HTTP {}: {}", status, text));
    }

    // Deserialize JSON
    serde_json::from_str::<TResp>(&text)
        .map_err(|e| format!("deserialize error: {e}; body: {text}"))
}

/* ---------- /v1/open types & convenience wrapper ---------- */

#[derive(Debug, Clone, Serialize)]
pub struct OpenFileReq<'a> {
    pub path: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_sha: Option<&'a str>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OpenFileResp {
    pub path: String,
    pub size_bytes: u64,
    pub char_count: usize,
    pub line_count: usize,
    /// Present only when content changed or client had no sha.
    pub content: Option<String>,
    pub sha256: String,
    pub unchanged: bool,
}

/// Convenience: POST /v1/open { path, client_sha }
pub async fn open_file(path: &str, client_sha: Option<&str>) -> Result<OpenFileResp, String> {
    let req = OpenFileReq { path, client_sha };
    post_json::<_, OpenFileResp>("/v1/open", &req).await
}
