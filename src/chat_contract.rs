#![feature(use_extern_macros)]

extern crate protobuf;

use std::collections::HashMap;

use ekiden_core_common::{Error, Result};
use ekiden_core_common::contract::{Address, Contract};

use chat_api::{ChatState, Snippet};

pub struct ChatContract {
    name: String,
    chat_text: Vec::<Message>, // Maps user to message
}

pub struct Message {
    pub sender: String,
    pub content: String,
}

impl Clone for Message {
    fn clone(&self) -> Message {
        Message {
            sender: self.sender.clone(),
            content: self.content.clone(),
        }
    }
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
                };
                ct.push(init_msg);
                ct
            },
        }
    }

    // Private Methods
    fn do_add_message(&mut self, msg: Message) -> Result<()> {
        self.chat_text.push(msg);
        return Ok(());
    }

    fn get_message(&self, index: usize) -> &Message {
        return self.chat_text.get(index).unwrap();
    }

    //Public Methods
    pub fn add_message(&mut self, msg: Message) -> Result<()> {
        self.do_add_message(msg)
    }

    pub fn fetch_messages(&self, index: usize) -> &[Message] {
        // Need to handle bad input case !!
        return &self.chat_text[index..];
    }
}

impl Contract<ChatState> for ChatContract {
    fn get_state(&self) -> ChatState {
        let mut state = ChatState::new();

        state.set_name(self.name.clone());

        let mut ret_val = Vec::<Snippet>::new();
        let msgs = self.chat_text.clone();
        for m in msgs {
            let mut s = Snippet::new();
            s.set_sender(m.sender);
            s.set_content(m.content);
            ret_val.push(s);
        }
        let ret_val = protobuf::RepeatedField::<Snippet>::from_vec(ret_val);
        state.set_messages(ret_val);

        state
    }

    fn from_state(state: &ChatState) -> ChatContract {
        ChatContract {
            name: state.get_name().to_string(),
            chat_text: {
                let mut ct = Vec::<Message>::new();
                for s in state.get_messages().clone() {
                    let m = Message {
                        sender: s.get_sender().clone().to_string(),
                        content: s.get_content().clone().to_string(),
                    };
                    ct.push(m);
                }

                ct
            }
        }
    }
}
