use rosc::{encoder, OscMessage, OscPacket};
use std::collections::VecDeque;
use std::sync::atomic::Ordering;
use std::{
    net::UdpSocket,
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use crate::shared_data::SharedData;
use crate::text_utils::split_into_chunks;

#[derive(Debug)]
#[allow(unused)]
pub enum Message {
    Text {
        content: String,
        immediate: bool,
        notify: bool,
    },
    Typing(bool),
}

pub fn hook_sender_thread(queue: Arc<Mutex<VecDeque<Message>>>, data: Arc<SharedData>, ip: &str, port: u16) {
    let vrchat_ip = ip;
    let vrchat_port = port;

    let socket = UdpSocket::bind("0.0.0.0:0").expect("Unable to bind socket");
    socket
        .connect((vrchat_ip, vrchat_port))
        .expect("Unable to connect to vrchat! Is it open?");

    loop {
        let message = {
            let mut queue = queue.lock().unwrap();
            queue.pop_front()
        };
        if let Some(msg) = message {
            match msg {
                Message::Text {
                    content,
                    immediate,
                    notify,
                } => {
                    data.is_sending.store(true, Ordering::SeqCst);
                    // Split the input into chunks (max 144 characters, split by words)
                    let chunks = split_into_chunks(&content, 134);

                    // Send each chunk as an OSC message
                    for chunk in chunks {
                        let osc_message = OscMessage {
                            addr: "/chatbox/input".to_string(), // OSC address for VRChat chatbox input
                            args: vec![
                                rosc::OscType::String(chunk.clone()),
                                rosc::OscType::Bool(immediate),
                                rosc::OscType::Bool(notify),
                            ],
                        };

                        // Wrap the message in an OSC packet
                        let packet = OscPacket::Message(osc_message);

                        // Encode the packet into bytes
                        let encoded_packet =
                            encoder::encode(&packet).expect("Failed to encode OSC packet");

                        // Send the encoded packet over UDP
                        socket
                            .send(&encoded_packet)
                            .expect("Unable to send packet! Is vrchat still open?");

                        println!("Sent message: \"{chunk}\"");

                        if immediate == true {
                            sleep(Duration::from_secs(data.message_delay.load(Ordering::SeqCst)));
                        }
                    }
                    
                    data.is_sending.store(false, Ordering::SeqCst);
                }
                Message::Typing(on) => {
                    let osc_message = OscMessage {
                        addr: "/chatbox/typing".to_string(),
                        args: vec![rosc::OscType::Bool(on)],
                    };

                    // Wrap the message in an OSC packet
                    let packet = OscPacket::Message(osc_message);

                    // Encode the packet into bytes
                    let encoded_packet =
                        encoder::encode(&packet).expect("Failed to encode OSC packet");

                    // Send the encoded packet over UDP
                    socket
                        .send(&encoded_packet)
                        .expect("Unable to send packet! Is vrchat still open?");

                    println!("Set typing indicator to {on}");
                }
            }
        }
    }
}
