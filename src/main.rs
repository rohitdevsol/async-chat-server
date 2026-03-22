#![allow(dead_code,unused)]
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures::StreamExt;


#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server listening on the port 3000");


    loop {
        let (stream,_) = listener.accept().await.unwrap();
        println!("new connection ");

        let mut ws_stream = accept_async(stream).await.unwrap();
        println!("Websocket connected");

        while let Some(message) = ws_stream.next().await{
            let msg =message.unwrap();
            println!("Received {:?}",msg.to_text().unwrap());
        }
        

    }
}
