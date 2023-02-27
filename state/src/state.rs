use app_io::*;
use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};

#[metawasm]
pub mod metafns {
    pub type State = <ContractMetadata as Metadata>::State;

    pub fn pingers(state: State) -> Vec<ActorId> {
        state.pingers()
    }

    pub fn ping_count(state: State, actor: ActorId) -> u128 {
        state.ping_count(actor)
    }
}
