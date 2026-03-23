#![allow(dead_code,unused)]
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures::{StreamExt,SinkExt};
use tokio::sync::mpsc;
use crate::message::{ClientId, Event};
use std::collections::HashMap;
pub mod message;
pub mod connection;
pub mod server;
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server listening on the port 3000");

    let (tx, mut rx) = mpsc::channel::<Event>(100);

    let mut clients : HashMap<ClientId,mpsc::Sender<String>> = HashMap::new();

     tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            
            match event {
                Event::NewClient(id,client_tx)=>{
                    clients.insert(id,client_tx);
                    println!("Client {} connected",{id});
                },
                Event::Message(id,msg)=> {
                    println!("Client {}:{}",id, msg);

                    // broadcast from here
                    for (other_id,tx) in clients.iter(){
                        if *other_id != id {
                            let _ = tx.send(msg.clone()).await;
                        }
                    }
                },
                Event::Disconnect(id)=>{
                    clients.remove(&id);
                    println!("Client {} disconnected",id);
                }
            }

        }
    });

     let mut next_id:ClientId = 0;

    loop {
        let (stream,_) = listener.accept().await.unwrap();
        println!("new connection ");

        let client_id = next_id;
        next_id+=1;

        let tx_clone = tx.clone();

        tokio::spawn(async move {
            connection::handle_connection(stream, client_id, tx_clone).await;
        });

    }

   }
