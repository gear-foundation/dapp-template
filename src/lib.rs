#![no_std]

use gstd::{collections::HashMap, errors::Result, msg, prelude::*, ActorId};
use template_io::*;

static mut STATE: Option<HashMap<ActorId, u128>> = None;

fn state_mut() -> &'static mut HashMap<ActorId, u128> {
    unsafe { STATE.as_mut().expect("state isn't initialized") }
}

// The `init()` entry point.
#[no_mangle]
extern fn init() {
    unsafe { STATE = Some(HashMap::new()) }
}

// The `handle()` entry point.
#[no_mangle]
extern fn handle() {
    process_handle().expect("failed to load, decode, encode, or reply from `handle()`")
}

fn process_handle() -> Result<()> {
    let payload = msg::load()?;

    if let PingPong::Ping = payload {
        let pingers = state_mut();

        pingers
            .entry(msg::source())
            .and_modify(|ping_count| *ping_count = ping_count.saturating_add(1))
            .or_insert(1);

        reply(PingPong::Pong)?;
    }

    Ok(())
}

// The `state()` entry point.
#[no_mangle]
extern fn state() {
    reply(State::from_iter(state_mut().clone())).expect("failed to encode or reply from `state()`")
}

fn reply(payload: impl Encode) -> Result<()> {
    msg::reply(payload, 0).map(|_| ())
}
