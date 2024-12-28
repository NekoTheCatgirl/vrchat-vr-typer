use std::{
    collections::VecDeque,
    sync::{atomic::Ordering, Arc, Mutex},
};

use iced::{
    widget::{button, center, checkbox, column, row, text, text_input},
    Alignment::{self, Center},
    Element, Size, Task,
};

use crate::{osc::Message, shared_data::SharedData};

/// Entry point for the user interface, takes the message queue
pub fn render_ui(queue: Arc<Mutex<VecDeque<Message>>>, data: Arc<SharedData>) {
    iced::application("Vrchat Typer", VrchatTyper::update, VrchatTyper::view)
        .window(iced::window::Settings {
            size: Size::new(500.0, 150.0),
            ..Default::default()
        })
        .resizable(false)
        .run_with(|| VrchatTyper::new(queue, data))
        .expect("Unable to create UI");
}

#[derive(Default)]
struct VrchatTyper {
    queue: Arc<Mutex<VecDeque<Message>>>,
    data: Arc<SharedData>,
    message_delay: u64,
    input_value: String,
    sending_state: bool,
    notify_state: bool,
}

impl VrchatTyper {
    fn new(
        queue: Arc<Mutex<VecDeque<Message>>>,
        data: Arc<SharedData>,
    ) -> (Self, Task<ApplicationMessage>) {
        (
            Self {
                queue,
                data,
                message_delay: 3,
                ..Default::default()
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: ApplicationMessage) {
        match message {
            ApplicationMessage::InputSent => {
                // Send the message...
                {
                    let mut queue = self.queue.lock().unwrap();
                    queue.push_back(Message::Text {
                        content: self.input_value.clone(),
                        immediate: true,
                        notify: self.notify_state,
                    });
                }
                self.input_value.clear();
            }
            ApplicationMessage::InputChanged(text) => {
                // The big one...
                let osc_sending = self.data.is_sending.load(Ordering::SeqCst);
                if osc_sending == false {
                    let mut queue = self.queue.lock().unwrap();
                    queue.push_back(Message::Typing(true));
                }
                self.input_value = text;

                if self.sending_state == true && self.input_value.len() < 140 {
                    {
                        let mut queue = self.queue.lock().unwrap();
                        queue.push_back(Message::Text {
                            content: self.input_value.clone(),
                            immediate: false,
                            notify: self.notify_state,
                        });
                    }
                }
            }
            ApplicationMessage::SendingToggled(send) => self.sending_state = send,
            ApplicationMessage::NotifyToggled(notify) => self.notify_state = notify,
            ApplicationMessage::Increment => {
                self.message_delay += 1;
                if self.message_delay > 20 {
                    self.message_delay = 20;
                }
                self.data.message_delay.store(self.message_delay, Ordering::SeqCst);
            }
            ApplicationMessage::Decrement => {
                self.message_delay -= 1;
                if self.message_delay < 3 {
                    self.message_delay = 3;
                }
                self.data.message_delay.store(self.message_delay, Ordering::SeqCst);
            }
        }
    }

    fn view(&self) -> Element<ApplicationMessage> {
        let input = text_input("Enter text...", &self.input_value)
            .on_input(ApplicationMessage::InputChanged)
            .on_submit(ApplicationMessage::InputSent)
            .padding(10);

        let sending = checkbox("Send while typing", self.sending_state)
            .on_toggle(ApplicationMessage::SendingToggled);
        let notify =
            checkbox("Notify", self.notify_state).on_toggle(ApplicationMessage::NotifyToggled);

        let counter = row![
            button("Increase Delay").on_press(ApplicationMessage::Increment),
            text(self.message_delay).size(20),
            button("Lower Delay").on_press(ApplicationMessage::Decrement)
        ]
        .padding(20)
        .align_y(Center);

        let toggles_row = row![sending, notify].spacing(20);

        let content = column![input, toggles_row, counter]
            .spacing(10)
            .align_x(Alignment::Center);

        center(content).into()
    }
}

#[derive(Debug, Clone)]
enum ApplicationMessage {
    InputSent,
    InputChanged(String),
    SendingToggled(bool),
    NotifyToggled(bool),
    Increment,
    Decrement,
}
