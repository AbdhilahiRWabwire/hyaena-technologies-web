mod arguments;
mod configuration;
mod errors;
mod eventing;
mod hypertext_transfer;
mod json;
mod logging;
mod routing;

mod service;
use service::application_service::web_service;

mod yaml;

// Main Entry Point
fn main() -> () {
    web_service();

    return ();
}
