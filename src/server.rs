//! Websocket server

use std::{thread, fmt::Display};
use futures::executor;

use broadcaster::BroadcastChannel;
use midly::num::u7;
use websocket::{sync::Server, OwnedMessage};

use crate::tuner::Monzo;

const WEBSOCKET_ADDR: &str = "127.0.0.1:8765";

/// This is the message that gets sent to the JI lattice visualizer.
#[derive(Clone)]
pub enum VisualizerMessage {
    NoteOn {
        /// Number of 12 edo semitones from A4 of the note.
        edosteps_from_a4: i32,
        /// Note velocity.
        velocity: u7,
        monzo: Monzo,
    },
    NoteOff {
        edosteps_from_a4: i32,
        velocity: u7,
    },
    CC {
        controller: u7,
        value: u7,
    }
}

impl Display for VisualizerMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VisualizerMessage::NoteOn { edosteps_from_a4, velocity, monzo } => {
                let monzo_str = monzo.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(":");
                write!(f, "on:{}:{}:{}", edosteps_from_a4, velocity, monzo_str)
            },
            VisualizerMessage::NoteOff { edosteps_from_a4, velocity } => {
                write!(f, "off:{}:{}", edosteps_from_a4, velocity)
            },
            VisualizerMessage::CC { controller, value } => {
                write!(f, "cc:{}:{}", controller, value)
            }
        }
    }
}

/// Starts the websocket server at [`WEBSOCKET_ADDR`]
///
/// Returns a clonable broadcast channel that can be used to send messages to all connected clients.
///
/// (It can also receive the messages it sends, but that's not necessary)
pub fn start_websocket_server() -> BroadcastChannel<VisualizerMessage> {
    println!("Starting websocket server...");

    // clonable broadcast channel (messages sent by one end received by all ends, any channel can send messages)
    let chan: BroadcastChannel<VisualizerMessage> = BroadcastChannel::new();

    let server = Server::bind(WEBSOCKET_ADDR).expect("Failed to bind websocket server");

    let chan_recv = chan.clone();
    thread::spawn(move || {
        let chan_recv = chan_recv; // move chan_recv into request handler thread.

        for request in server.filter_map(Result::ok) {
            let mut chan_recv = chan_recv.clone(); // clone chan_recv for each connection.
            // Spawn a new thread for each connection.
            thread::spawn(move || {
                let mut client = request.accept().unwrap();

                let ip = client.peer_addr().unwrap();

                println!("Connection from {}", ip);

                while let Some(msg) = executor::block_on(chan_recv.recv()) {
                    let msg_str = msg.to_string();
                    let res = client.send_message(&OwnedMessage::Text(msg_str));
                    if let Err(e) = res {
                        println!("Closing connection to {ip}: {e}");
                        break;
                    }
                }

                if let Err(e) = client.shutdown() {
                    println!("WARN: Failed to close connection to {ip}: {e}");
                }
            });
        }
    });

    chan
}
