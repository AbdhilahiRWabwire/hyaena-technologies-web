#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::{
    net::{
        IpAddr, 
        Ipv4Addr, 
        SocketAddr, 
        TcpListener, 
        TcpStream
    },
    string::String
};

// Hypertext Transfer Protocol Definition
pub struct HypertextTransferProtocol {
    pub body: T,
    pub headers: String,
    pub method: String,
    pub status_code: String,
    pub version: String
}

// Hypertext Transfer Protocol Client Definition
pub struct HypertextClient {

}

// Hypertext Transfer Protocol Connection Definition
pub struct HypertextConnection {

}

// Hypertext Transfer Protocol Server Definition
pub struct HypertextServer {

}