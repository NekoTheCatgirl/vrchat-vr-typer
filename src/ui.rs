use std::{collections::VecDeque, sync::{Arc, Mutex}};

use crate::{osc::Message, shared_data::SharedData};

/// Entry point for the user interface, takes the message queue
pub fn render_ui(_queue: Arc<Mutex<VecDeque<Message>>>, _data: Arc<SharedData>) {
    
}