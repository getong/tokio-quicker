#![allow(clippy::unused_io_amount)]
use std::fs;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_quicker::connection::Incoming;
use tokio_quicker::error::Result;
use tokio_quicker::QuicListener;

#[tokio::main]
async fn main() -> Result<()> {
    //simple_logger::SimpleLogger::new().init().unwrap();

    let mut listener = QuicListener::bind("127.0.0.1:4433").await?;

    while let Ok(mut connection) = listener.accept().await {
        tokio::spawn(async move {
            match connection.incoming().await.unwrap() {
                Incoming::Bidi(mut stream) => {
                    let mut buf = [0; u16::MAX as usize];
                    let len = stream.read(&mut buf).await.unwrap();
                    let path = String::from_utf8_lossy(&buf[..len]);
                    println!("Reading: {path}");
                    let string =
                        fs::read_to_string(path.to_string()).unwrap_or_else(|err| err.to_string());
                    stream.write(string.as_bytes()).await.unwrap();
                    stream.flush().await.unwrap();
                    stream.shutdown().await.unwrap();
                    println!("Shutdown!")
                }
                _ => {
                    println!("Stream is not writable!")
                }
            }
        });
    }
    Ok(())
}
