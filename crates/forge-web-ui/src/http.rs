use web_sys::console; // <-- add this at the top

/// Generic JSON POST helper for browser/WASM.
pub async fn post_json<TReq, TResp>(path: &str, body: &TReq) -> Result<TResp, String>
where
    TReq: Serialize,
    TResp: DeserializeOwned,
{
    let win = window().ok_or_else(|| "window() not available".to_string())?;

    let json = serde_json::to_string(body).map_err(|e| format!("serialize error: {e}"))?;

    console::log_1(&format!("🌐 POST {} body = {}", path, json).into()); // <-- log request

    let init = RequestInit::new();
    init.set_method("POST");
    init.set_mode(RequestMode::SameOrigin);

    let headers = Headers::new().map_err(|e| format!("headers error: {:?}", e))?;
    headers.set("Content-Type", "application/json")
        .map_err(|e| format!("set header error: {:?}", e))?;
    init.set_headers(&headers);
    init.set_body(&wasm_bindgen::JsValue::from_str(&json));

    let req = Request::new_with_str_and_init(path, &init)
        .map_err(|e| format!("request init error: {:?}", e))?;

    let resp_val = JsFuture::from(win.fetch_with_request(&req))
        .await
        .map_err(|e| format!("fetch error: {:?}", e))?;

    let resp: Response = resp_val.dyn_into().map_err(|_| "bad response".to_string())?;
    let status = resp.status();
    let ok = resp.ok();

    let text_js = JsFuture::from(resp.text().map_err(|e| format!("resp.text() error: {:?}", e))?)
        .await
        .map_err(|e| format!("read body error: {:?}", e))?;
    let text = text_js.as_string().unwrap_or_default();

    console::log_1(
        &format!("⬅️  POST {} → HTTP {} (ok={})\n{}", path, status, ok, text).into(),
    ); // <-- log response

    if !ok {
        return Err(format!("HTTP {}: {}", status, text));
    }

    serde_json::from_str::<TResp>(&text)
        .map_err(|e| format!("deserialize error: {e}; body: {text}"))
}

/// Minimal GET helper that returns plain text (useful for pings, etc.)
pub async fn get_text(path: &str) -> Result<String, String> {
    let win = window().ok_or_else(|| "window() not available".to_string())?;

    console::log_1(&format!("🌐 GET {}", path).into()); // <-- log request

    let init = RequestInit::new();
    init.set_method("GET");
    init.set_mode(RequestMode::SameOrigin);

    let req = Request::new_with_str_and_init(path, &init)
        .map_err(|e| format!("request init error: {:?}", e))?;

    let resp_val = JsFuture::from(win.fetch_with_request(&req))
        .await
        .map_err(|e| format!("fetch error: {:?}", e))?;

    let resp: Response = resp_val.dyn_into().map_err(|_| "bad response".to_string())?;
    let status = resp.status();
    let ok = resp.ok();

    let text_js = JsFuture::from(resp.text().map_err(|e| format!("resp.text() error: {:?}", e))?)
        .await
        .map_err(|e| format!("read body error: {:?}", e))?;
    let text = text_js.as_string().unwrap_or_default();

    console::log_1(
        &format!("⬅️  GET {} → HTTP {} (ok={})\n{}", path, status, ok, text).into(),
    ); // <-- log response

    if !ok {
        return Err(format!("HTTP {}: {}", status, text));
    }
    Ok(text)
}
