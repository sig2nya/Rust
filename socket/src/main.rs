use tokio::net::{
    TcpListener,
    TcpStream
};
use tokio::io::{
    AsyncReadExt,
    AsyncWriteExt
};
use unicode_normalization::UnicodeNormalization;
use std::error::Error;
use std::net::SiocketAddr;

fn hex_dump(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" ")
}
