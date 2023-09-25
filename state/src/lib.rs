#![no_std]

use template_io::*;

#[gmeta::metawasm]
pub mod metafns {
    pub type State = template_io::State;

    pub fn query(state: State, query: StateQuery) -> StateQueryReply {
        match query {
            StateQuery::Pingers => {
                StateQueryReply::Pingers(state.iter().map(|(pinger, _)| *pinger).collect())
            }
            StateQuery::PingCount(actor) => StateQueryReply::PingCount(
                state
                    .iter()
                    .find_map(|(some_actor, count)| (some_actor == &actor).then_some(count))
                    .copied()
                    .unwrap_or_default(),
            ),
        }
    }
}
