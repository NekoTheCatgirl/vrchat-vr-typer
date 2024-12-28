use std::sync::atomic::{AtomicBool, AtomicU64};

#[derive(Default)]
pub struct SharedData {
    pub is_sending: AtomicBool,
    /// Must be at least 3
    pub message_delay: AtomicU64,
}