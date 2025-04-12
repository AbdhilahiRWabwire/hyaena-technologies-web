mod arguments;
mod configuration;
mod hypertext;

mod service;
use service::application_service::web_service;

mod utility;

// Main Entry Point
fn main() -> () {
    web_service();

    return ();
}
