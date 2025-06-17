mod arguments;
mod configuration;
mod hypertext_transfer;
mod routing;

mod service;
use service::application_service::web_service;

mod utility;
mod yaml;

// Main Entry Point
fn main() -> () {
    web_service();

    return ();
}
