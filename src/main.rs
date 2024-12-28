mod osc;
mod text_utils;

use std::{collections::VecDeque, sync::{Arc, Mutex}, thread::{self, sleep}, time::Duration};

use osc::{hook_sender_thread, Message};

fn main() {
    let queue = Arc::new(Mutex::new(VecDeque::new()));
    
    let worker_queue = queue.clone();
    thread::spawn(move || {
        hook_sender_thread(&worker_queue);
    });

    {
        let mut queue = queue.lock().unwrap();
        queue.push_back(Message::Typing(true));
    }

    send_message(&queue, "Sending message 1/7");
    send_message(&queue, "Sending message 2/7");
    send_message(&queue, "Sending message 3/7");
    send_message(&queue, "Sending message 4/7");
    send_message(&queue, "Sending message 5/7");
    send_message(&queue, "Sending message 6/7");
    send_message(&queue, "Sending message 7/7");

    {
        let mut queue = queue.lock().unwrap();
        queue.push_back(Message::Typing(false));
    }

    loop {
        sleep(Duration::from_secs(1));
    }
}

fn send_message(queue: &Arc<Mutex<VecDeque<Message>>>, message: &str) {

    let mut queue = queue.lock().unwrap();
    queue.push_back(Message::Text { content: message.to_string(), immediate: true, notify: true });
    println!("Queued up \"{message}\"");
}