#![allow(dead_code,unused)]
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures::StreamExt;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server listening on the port 3000");

    let (tx, mut rx) = mpsc::channel(100);


    loop {
        let (stream,_) = listener.accept().await.unwrap();
        println!("new connection ");


        // handle multiple clients
        let tx_clone = tx.clone();
        tokio::spawn(async move{
            let mut ws_stream = accept_async(stream).await.unwrap();
              println!("Websocket connected");

            while let Some(message) = ws_stream.next().await{
                let msg = message.unwrap().to_string();
                // println!("Received {}",msg.to_text().unwrap());
                tx_clone.send(msg).await.unwrap();
            }

        });

              

    }

    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            println!("Server event received");
        }
    });
}
