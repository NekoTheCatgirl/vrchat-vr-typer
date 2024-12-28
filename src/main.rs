mod osc;
mod text_utils;
mod ui;

use std::{collections::VecDeque, sync::{Arc, Mutex}, thread};

use osc::hook_sender_thread;
use ui::render_ui;

fn main() {
    let queue = Arc::new(Mutex::new(VecDeque::new()));
    
    let worker_queue = queue.clone();
    thread::spawn(move || {
        hook_sender_thread(&worker_queue);
    });

    render_ui(&queue);
}