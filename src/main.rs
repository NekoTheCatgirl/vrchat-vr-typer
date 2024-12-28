mod osc;
mod text_utils;
mod ui;
mod shared_data;
mod args;

use std::{collections::VecDeque, sync::{atomic::Ordering, Arc, Mutex}, thread};

use args::Arguments;
use clap::Parser;
use osc::hook_sender_thread;
use shared_data::SharedData;
use ui::render_ui;

fn main() {
    let args = Arguments::parse();
    let queue = Arc::new(Mutex::new(VecDeque::new()));
    let data = Arc::new(SharedData::default());

    data.message_delay.store(3, Ordering::SeqCst);
    
    let worker_queue = queue.clone();
    let worker_data = data.clone();
    thread::spawn(move || {
        hook_sender_thread(worker_queue, worker_data, &args.ip, args.port);
    });

    render_ui(queue, data);
}