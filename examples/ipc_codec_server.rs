use std::error::Error;

use futures::{SinkExt, StreamExt};
use parity_tokio_ipc::{IpcSecurity, OnConflict, SecurityAttributes, ServerId};
use transport_async::codec::{Codec, SerdeCodec};
use transport_async::transport::codec::CodecTransport;
use transport_async::transport::ipc;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let incoming = ipc::create_endpoint(
        ServerId("test"),
        SecurityAttributes::allow_everyone_create().expect("Failed to set security attributes"),
        OnConflict::Overwrite,
    )?;

    let mut transport =
        CodecTransport::new(incoming, SerdeCodec::<usize, usize>::new(Codec::Bincode));
    while let Some(result) = transport.next().await {
        match result {
            Ok(mut stream) => {
                tokio::spawn(async move {
                    loop {
                        if let Some(Ok(req)) = stream.next().await {
                            println!("ping {req}");
                            stream.send(req + 1).await.unwrap();
                        }
                    }
                });
            }
            _ => unreachable!("ideally"),
        }
    }

    Ok(())
}