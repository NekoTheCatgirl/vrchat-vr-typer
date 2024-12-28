mod osc;
mod text_utils;

use std::{collections::VecDeque, sync::{Arc, Mutex}, thread};

use osc::hook_sender_thread;

fn main() {
    let queue = Arc::new(Mutex::new(VecDeque::new()));
    
    let worker_queue = queue.clone();
    thread::spawn(move || {
        hook_sender_thread(&worker_queue);
    });


}