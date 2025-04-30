mod arguments;
mod configuration;
mod hypertext_markup;
mod hypertext_transfer;

mod service;
use service::application_service::web_service;

mod utility;

// Main Entry Point
fn main() -> () {
    web_service();

    return ();
}
