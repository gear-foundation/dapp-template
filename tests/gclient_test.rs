use app::WASM_BINARY_OPT;
use app_io::*;
use gclient::{EventProcessor, GearApi};
use gstd::prelude::*;

#[tokio::test]
#[ignore]
async fn gclient_test() {
    let client = GearApi::dev()
        .await
        .expect("The node must be running for a gclient test");
    let mut listener = client.subscribe().await.unwrap();

    let mut gas_limit = client
        .calculate_upload_gas(None, WASM_BINARY_OPT.into(), vec![], 0, true, None)
        .await
        .unwrap()
        .min_limit;
    let (mut message_id, program_id, _) = client
        .upload_program_bytes(WASM_BINARY_OPT, gclient::bytes_now(), [], gas_limit, 0)
        .await
        .unwrap();

    assert!(listener
        .message_processed(message_id)
        .await
        .unwrap()
        .succeed());

    gas_limit = client
        .calculate_handle_gas(None, program_id, PingPong::Ping.encode(), 0, true, None)
        .await
        .unwrap()
        .min_limit;
    (message_id, _) = client
        .send_message(program_id, PingPong::Ping, gas_limit, 0)
        .await
        .unwrap();

    let (_, raw_reply, _) = listener.reply_bytes_on(message_id).await.unwrap();

    assert_eq!(
        PingPong::Pong,
        Decode::decode(&mut raw_reply.unwrap().as_slice()).unwrap()
    );
}
