use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn main() {
    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");
    server.set_nonblocking(true).expect("Failed to initialize nonblocking");

    let mut clients = vec![];  
    
    let (tx, rx) = mpsc::channel::<String>();

    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);
            let tx = tx.clone();
            clients.push(socket.try_clone().expect("Failed to clone client")); 
            thread::spawn(move || loop {
                let mut buff: Vec<u8> = vec![0; MSG_SIZE];
                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        let msg = buff.into_iter().take_while(|&x | x != 0).collect::<Vec<_>>();
                        let str_msg = String::from_utf8(msg).expect("Invalid ut8 msg");
                        println!("{}: {:?}", addr, str_msg);
                        tx.send(str_msg).expect("failed to send message");
                    },
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("closing connection with {}", addr);
                        break;
                    }
                } 
                sleep();
            });
        }

        if let Ok(msg) = rx.try_recv() {
            clients = clients.into_iter().filter_map( |mut client | {
                 let mut buff: Vec<u8> = msg.clone().into_bytes();
                    buff.resize(MSG_SIZE, 0); 
                    client.write_all(&buff).map(|_| client).ok()
            }).collect::<Vec<_>>();
        }
        sleep(); 
    }
}  

fn sleep () {
    thread::sleep(Duration::from_millis(100))
}