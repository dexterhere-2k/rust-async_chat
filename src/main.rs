
use std::net::SocketAddr;

use tokio::{io::{
    AsyncBufRead, AsyncBufReadExt, AsyncWriteExt, BufReader
},
net::TcpListener,
sync::broadcast};


#[tokio::main]
async fn main(){
    let listener=TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("chat server listening on 127.0.0.1:8080");
    let(tx,_rx)=broadcast::channel::<(SocketAddr,String)>(10);

    loop{
        let (mut socket,addr)=listener.accept().await.unwrap();
        println!("New Client Connected");
        let tx=tx.clone();
        let mut rx=tx.subscribe();
        tokio::spawn(async move{
            let (reader,mut writer)=socket.split();

            let mut reader=BufReader::new(reader);
            let mut line=String::new();



            loop{
                tokio::select! {
                    result=reader.read_line(&mut line)=>{
                        if result.unwrap()==0{
                            break;
                        }
                        let msg=(addr,format!("{} : {}",addr,line));
                        tx.send(msg).unwrap();
                        line.clear();
                    }

                    result=rx.recv()=>{
                        
                        let (sender_addr,msg)=result.unwrap();
                        if !(sender_addr==addr){

                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                        else{
                            writer.write_all("Sent\n".as_bytes()).await.unwrap();
                        }
                    }
                }
            }
    
        });

}}