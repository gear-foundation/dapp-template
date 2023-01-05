#![no_std]

use app_io::*;
use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};

#[metawasm]
pub trait Metawasm {
    type State = <AppMetadata as Metadata>::State;

    fn pingers(state: Self::State) -> Vec<ActorId> {
        app_io::pingers(state)
    }

    fn ping_count(actor: ActorId, state: Self::State) -> u128 {
        app_io::ping_count(state, actor)
    }
}
