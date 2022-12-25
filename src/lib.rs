#![no_std]

use gstd::{debug, metadata, msg, prelude::*};

#[derive(Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Payload {
    question: String,
    answer: u8,
}

impl Default for Payload {
    fn default() -> Self {
        Self {
            question: "life-universe-everything".into(),
            answer: 42,
        }
    }
}

metadata! {
    title: "App",
    handle:
        input: Payload,
        output: u8,
}

#[no_mangle]
extern "C" fn init() {
    let payload = String::from_utf8(msg::load_bytes().expect("Failed to load a message"))
        .expect("Invalid init message");
    debug!("init(): {}", payload);
}

#[no_mangle]
extern "C" fn handle() {
    debug!("handle()");
    let payload: Payload = msg::load().expect("Unable to decode payload");
    if payload.question == "life-universe-everything" {
        msg::reply(payload.answer, 0).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::Payload;
    use gtest::{Log, Program, System};

    #[test]
    fn question_answer() {
        let system = System::new();
        system.init_logger();

        let program = Program::current(&system);

        let res = program.send_bytes(12, "Let's start");
        assert!(res.log().is_empty());

        let res = program.send(12, Payload::default());
        let log = Log::builder().source(1).dest(12).payload(42_u8);
        assert!(res.contains(&log));
    }
}
