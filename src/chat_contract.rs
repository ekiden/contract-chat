use std::collections::HashMap;

use ekiden_core_common::{Error, Result};
use ekiden_core_common::contract::{Address, Contract};

use chat_api::ChatState;

pub struct ChatContract {
    name: String,
    chat_text: Vec::<Message>; // Maps user to message
}

pub struct Message {
    sender: String,
    content: String,
}

impl ChatContract {
    pub fn new(
        chat_name: String,
    ) -> ChatContract {
        ChatContract {
            name: chat_name.clone(),
            chat_text: {
                let mut ct = Vec::<Message>::new();
                let mut init_msg = Message {
                    sender: String::from("NetBot"),
                    content: String::from("This is the beginning of the chat!"),
                }
                ct.push(init_msg),
                ct
            },
        }
    }

    // Private Methods
    fn do_add_message(&mut self, msg: &Message) -> Result<()> {
        self.chat_text.push(msg);
        return Ok(());
    }

    fn get_message(&self, index: u64) -> &Message {
        return self.chat_text.get(index)?;
    }

    //Public Methods
    fn add_message(&mut self, msg: &Message) -> Result<()> {
        self.do_add_message(msg)
    }
}

impl Contract<ChatState> for ChatContract {
    fn get_state(&self) -> ChatState {
        let mut state = ChatState::new();
        state.set_name(self.name.clone());
        state.set_chat_text(self.name.chat_text.clone());

        state
    }

    fn from_state(state: &ChatState) -> ChatContract {
        ChatContract {
            name: state.get_name().to_string(),
            chat_text: state.get_chat_text().clone(),
        }
    }
}
