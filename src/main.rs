#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = "http://example.org".parse::<hyper::Uri>()?;
    let host = url.host().expect("url to have a host");
    let port = url.port_u16().unwrap_or(80);
    let address = format!("{host}:{port}");
    let stream = TcpStream::connect(address).await?;
    let io = TokioIo::new(stream);
    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;

    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    let authority = url.authority().unwrap().clone();

    let req = Request::builder()
        .uri(url)
        .header(hyper::header::HOST, authority.as_str())
        .body(Empty::<Bytes>::new())?;

    let mut response = sender.send_request(req).await?;

    println!(
        "Got HTTP {}, with headers: {:#?}",
        response.status(),
        response.headers()
    );

    let body = response.body();

    println!("Body: {:?}", body);

    // while let Some(next) = response.frame().await {
    //     let frame = next?;
    //     if let Some(chunk) = frame.data_ref() {
    //         stdout().write_all(chunk).await?;
    //     }
    // }
    Ok(())
}

// use http_body_util::BodyExt;
use http_body_util::Empty;
use hyper::body::Bytes;
use hyper::Request;
use hyper_util::rt::TokioIo;
// use tokio::io::stdout;
// use tokio::io::AsyncWriteExt as _;
use tokio::net::TcpStream;
