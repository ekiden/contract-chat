rpc_api! {
    metadata {
        name = chat;
        version = "0.1.0";
        client_attestation_required = false;
    }

    rpc create(CreateRequest) -> CreateResponse;

    rpc add_message(AddMessageRequest) -> AddMessageResponse;

    rpc fetch_messages(FetchMessagesRequest) -> FetchMessagesResponse;
}
