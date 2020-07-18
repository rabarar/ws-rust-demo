use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::protocol::frame::coding::CloseCode::Normal;
use tungstenite::protocol::CloseFrame;
use tungstenite::server::accept;

fn main() {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    for stream in server.incoming() {
        println!("received incoming connection...");
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();

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
}

