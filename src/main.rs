mod arguments;
mod configuration;
mod errors;
mod hypertext_transfer;
mod json;
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
