#![feature(use_extern_macros)]

#[macro_use]
extern crate clap;
extern crate futures;
extern crate rand;
extern crate tokio_core;

#[macro_use]
extern crate client_utils;
extern crate ekiden_core_common;
extern crate ekiden_rpc_client;

extern crate chat_api;

use clap::{App, Arg};
use futures::future::Future;
use rand::{thread_rng, Rng};

use ekiden_rpc_client::create_client_rpc;
use chat_api::{with_api, ChatState, AddMessageRequest, AddMessageResponse, 
               CreateRequest, CreateResponse, FetchMessagesRequest, FetchMessagesResponse, Snippet};

with_api! {
    create_client_rpc!(chat, chat_api, api);
}

fn init<Backend>(client: &mut chat::Client<Backend>, _runs: usize, _threads: usize)
where
    Backend: ekiden_rpc_client::backend::ContractClientBackend,
{
    // Create new chat contract
    let mut request = CreateRequest::new();
    request.set_chat_name("MyChat1".to_string());

    client.create(request).wait().unwrap();
}

fn scenario<Backend>(client: &mut chat::Client<Backend>)
where
    Backend: ekiden_rpc_client::backend::ContractClientBackend,
{
    client.add_message({
        let mut request = AddMessageRequest::new();
        request.set_chat_name("MyChat1".to_string());
        request.set_sender("andy".to_string());
        request.set_content("Never make major decisions when you're reeling from a loss".to_string());
        request
    })
    .wait()
    .unwrap();

    let response = client.fetch_messages({
        let mut request = FetchMessagesRequest::new();
        request.set_index(0);
        request
    })
    .wait()
    .unwrap();

    let response = response.get_messages();

    for s in response {
        let sender = s.get_sender();
        let content = s.get_content();
        println!("{}: {}", sender, content);
    }
}

fn main() {
    let mut client = contract_client!(chat);
    init(&mut client, 1, 1);
    scenario(&mut client);
}