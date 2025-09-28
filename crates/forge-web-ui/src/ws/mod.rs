// Public WS façade: keep imports elsewhere stable.
mod connect;
mod handlers;
mod send;
mod util;
mod status;

pub use connect::connect_ws;
pub use send::send_intent;
pub use status::set_status;
