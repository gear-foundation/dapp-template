#![no_std]

use gmeta::{InOut, Metadata};
use gstd::{prelude::*, ActorId};

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum PingPong {
    Ping,
    Pong,
}

pub struct AppMetadata;

impl Metadata for AppMetadata {
    type Init = ();
    type Handle = InOut<PingPong, PingPong>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Vec<(ActorId, u128)>;
}

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum AppStateQuery {
    AllState,
    Pingers,
    PingCount(ActorId),
}

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Debug)]
pub enum AppStateQueryReply {
    AllState(<AppMetadata as Metadata>::State),
    Pingers(Vec<ActorId>),
    PingCount(u128),
}

pub fn pingers(state: <AppMetadata as Metadata>::State) -> Vec<ActorId> {
    state.into_iter().map(|pingers| pingers.0).collect()
}

pub fn ping_count(state: <AppMetadata as Metadata>::State, actor: ActorId) -> u128 {
    state
        .iter()
        .find_map(|(pinger, ping_count)| (*pinger == actor).then_some(*ping_count))
        .unwrap_or_default()
}
