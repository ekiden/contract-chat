#![feature(use_extern_macros)]

extern crate protobuf;

extern crate ekiden_core_common;
extern crate ekiden_core_trusted;

#[macro_use]
extern crate chat_api;

mod chat_contract;

use chat_api::{with_api, ChatState, AddMessageRequest, AddMessageResponse, 
               CreateRequest, CreateResponse, FetchMessagesRequest, FetchMessagesResponse};
use chat_contract::ChatContract;

use ekiden_core_common::Result;
use ekiden_core_common::contract::{with_contract_state, Address, Contract};
use ekiden_core_trusted::db::Db;
use ekiden_core_trusted::rpc::create_enclave_rpc;

// Create enclave RPC handlers.
with_api! {
    create_enclave_rpc!(api);
}

fn create(request: &CreateRequest) -> Result<CreateResponse> {
    let contract = ChatContract::new(request.get_chat_name().to_string());

    Db::instance().set("state", contract.get_state())?;

    Ok(CreateResponse::new());
}

fn add_message(request: &AddMessageRequest) -> Result<AddMessageResponse> {
    let state = Db::instance()::get("state")?;
    let state = with_contract_state(&state, |contract: &mut ChatContract| {
        let mut msg = Message {
            request.get_sender().to_string(),
            request.get_content().to_string(),
        };

        contract.add_message(&msg)?;

        Ok(())
    })?;

    Db::instance().set("state", state)?;

    Ok(AddMessageResponse::new())
}

fn fetch_messages(request: &FetchMessagesRequest) -> Result<FetchMessagesResponse> {
    let contract = ChatContract::from_state(&Db::instance().get("state")?);
    let msgs = contract.fetch_messages(request.get_index());

    let mut response = FetchMessagesResponse::new();
    response.set_messages(msgs);

    Ok(response)
}
