use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_tungstenite::accept_async;

use crate::message::{ClientId, Event};

pub async fn handle_connection(
    stream: TcpStream,
    client_id: ClientId,
    server_tx: mpsc::Sender<Event>,
) {
    // websocket upgrade happening here
    let ws_stream = accept_async(stream).await.unwrap();
    let (mut write, mut read) = ws_stream.split();

    // channel for this client
    let (client_tx, mut client_rx) = mpsc::channel::<String>(32);

    // register the client with the server
    let _ = server_tx.send(Event::NewClient(client_id, client_tx)).await;

    //writer task  (server to client)
    tokio::spawn(async move {
        while let Some(msg) = client_rx.recv().await {
            let _ = write.send(msg.into()).await;
        }
    });

    // client to server
    while let Some(message) = read.next().await {
        let msg = message.unwrap().to_string();

        let _ = server_tx.send(Event::Message(client_id, msg)).await;
    }

    //disconnect
    let _ = server_tx.send(Event::Disconnect(client_id)).await;
}
