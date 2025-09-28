use web_sys::window;

pub fn make_ws_url() -> String {
    let win = window().unwrap();
    let loc = win.location();
    let host = loc.host().unwrap_or_else(|_| "127.0.0.1:8080".into());
    let host_only = host.split(':').next().unwrap_or("127.0.0.1");
    format!("ws://{}:8787/ws", host_only)
}
