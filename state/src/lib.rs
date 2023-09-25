#![no_std]

use gstd::{prelude::*, ActorId};
use template_io::*;

#[gmeta::metawasm]
pub mod metafns {
    pub type State = template_io::State;

    pub fn query(state: State, query: StateQuery) -> StateQueryReply {
        match query {
            StateQuery::Pingers => StateQueryReply::Pingers(pingers(state)),
            StateQuery::PingCount(actor) => StateQueryReply::PingCount(ping_count(state, actor)),
        }
    }

    pub fn pingers(state: State) -> Vec<ActorId> {
        state.iter().map(|(pinger, _)| *pinger).collect()
    }

    pub fn ping_count(state: State, actor: ActorId) -> u128 {
        state
            .iter()
            .find_map(|(some_actor, count)| (some_actor == &actor).then_some(count))
            .copied()
            .unwrap_or_default()
    }
}
