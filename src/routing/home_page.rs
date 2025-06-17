use std::{fs, io::Write, net::TcpStream, path::PathBuf, primitive::usize};

use crate::hypertext_transfer::{
    http_headers::HTTP_CONTENT_LENGTH,
    http_status_codes::{HTTP_OK, HTTP_TWO_HUNDRED},
    http_versions::HTTP_VERSION_ONE,
};

// Application Home Page Route
pub fn home_route(mut transmission_stream: &TcpStream) -> () {
    let source_path: PathBuf = PathBuf::from("./web/source/main.html");
    let source_file: String = fs::read_to_string(source_path).unwrap();
    let content_length: usize = source_file.len();

    writeln!(
        transmission_stream,
        "{} {} {}",
        HTTP_VERSION_ONE.to_string(),
        HTTP_TWO_HUNDRED.to_string(),
        HTTP_OK.to_string()
    )
    .unwrap();
    writeln!(
        transmission_stream,
        "{}: {}",
        HTTP_CONTENT_LENGTH.to_string(),
        content_length
    )
    .unwrap();
    writeln!(transmission_stream, "{}", source_file).unwrap();

    return ();
}
