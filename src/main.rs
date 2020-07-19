use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::protocol::frame::coding::CloseCode::Normal;
use tungstenite::protocol::CloseFrame;
use tungstenite::server::accept;

use std::io::{Read};
use std::fs::File;
use std::sync::Arc;
use native_tls::{Identity, TlsAcceptor};

fn main() {

    let mut file = File::open("identity.pfx").unwrap();
    let mut identity = vec![];
    file.read_to_end(&mut identity).unwrap();
    let identity = Identity::from_pkcs12(&identity, "hunter2").unwrap();

    let acceptor = TlsAcceptor::new(identity).unwrap();
    let acceptor = Arc::new(acceptor);

    let listener = TcpListener::bind("127.0.0.1:8443").unwrap();



    for stream in listener.incoming() {
        println!("received incoming connection...");

        match stream {
            Ok(stream) => {
                let acceptor = acceptor.clone();
                spawn(move || {
                    let stream = acceptor.accept(stream).unwrap();
                    let mut websocket = accept(stream).unwrap();

                    loop {

                        let msg = match websocket.read_message() {
                            Err(why) => {
                                println!("read_message, reason: {:?}", why);
                                break;
                            }
                            Ok(msg) => msg,
                        };

                        // We do not want to send back ping/pong messages.
                        if msg.is_binary() || msg.is_text() {
                            match msg.to_text().unwrap().trim_end() {
                                "quit" => {
                                    println!("quitting");
                                    let cf = CloseFrame {
                                        code: Normal,
                                        reason: std::borrow::Cow::Borrowed("bye"),
                                    };

                                    println!("closing...");
                                    match websocket.close(Option::Some(cf)) {
                                        Err(why) => panic!("{:?}", why),
                                        Ok(_) => (),
                                    }

                                    match websocket.write_pending() {
                                        Err(why) => panic!("{:?}", why),
                                        Ok(rc) => println!("closed: {:?}...", rc),
                                    }

                                    break;
                                }
                                _ => {
                                    let resp = tungstenite::protocol::Message::Text(
                                        "hi there: ".to_string() + msg.to_text().unwrap(),
                                    );

                                    match websocket.write_message(resp) {
                                        Err(why) => panic!("{:?}", why),
                                        Ok(_) => (),
                                    }
                                }
                            }
                        }
                    }
                    println!("terminating thread...");
                });

            }
            Err(_e) => { /* connection failed */ }
        }
    }
}

