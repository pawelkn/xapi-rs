mod connection;
mod credentials;
mod data;
mod enums;
mod error;
mod socket;
mod stream;

pub use credentials::Credentials;
pub use data::*;
pub use enums::*;
pub use error::Error;
pub use socket::Socket;
pub use stream::Stream;

#[derive(Debug, Clone)]
pub struct XApi {
    pub socket: Socket,
    pub stream: Stream,
}

pub async fn connect(credentials: &Credentials) -> Result<XApi, Error> {
    let mut host = credentials.host.clone();
    if !host.starts_with("wss://") && !host.starts_with("ws://") {
        host.insert_str(0, "wss://");
    }

    let socket_url = format!("{}/{}", &host, &credentials.type_);
    let stream_url = format!("{}/{}Stream", &host, &credentials.type_);

    let socket = Socket::connect(&socket_url, credentials.safe).await?;
    let login = socket
        .login(&credentials.account_id, &credentials.password)
        .await?;

    let stream = Stream::connect(&stream_url, login.stream_session_id).await?;

    Ok(XApi { socket, stream })
}
