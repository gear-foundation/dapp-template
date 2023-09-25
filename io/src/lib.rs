#![no_std]

use gmeta::{InOut, Metadata, Out};
use gstd::{prelude::*, ActorId};

/// The contract metadata. Used by frontend apps & for describing the types of messages that can be
/// sent in contract's entry points. See also [`Metadata`].
pub struct ContractMetadata;

/// `()` means the contract doesn't process & reply messages at the above written entry point or
/// doesn't implement it.
impl Metadata for ContractMetadata {
    /// I/O types for the `init()` entry point.
    type Init = ();
    /// I/O types for the `handle()` entry point.
    ///
    /// Here the [`PingPong`] type is used for both incoming and outgoing messages.
    type Handle = InOut<PingPong, PingPong>;
    /// Types for miscellaneous scenarios.
    type Others = ();
    /// The input type for the `handle_reply()` entry point.
    type Reply = ();
    /// The output type for the `handle_signal()` entry point.
    type Signal = ();
    /// I/O types for the `state()` entry point.
    ///
    /// You can also specify just an output ([`Out`]) or input ([`In`](gmeta::In)) type, if both
    /// ([`InOut`]) are expected like here.
    type State = Out<State>;
}

pub type State = Vec<(ActorId, u128)>;

/// Replies with [`Pong`](PingPong::Pong) if received [`Ping`](PingPong::Ping).
#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum PingPong {
    Ping,
    Pong,
}

/// Queries the contract state.
///
/// Used in the `state` crate.
#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum StateQuery {
    /// Gets the list of actors who have [`ping`](PingPong::Ping)ed the contract.
    ///
    /// Returns [`StateQueryReply::Pingers`].
    Pingers,
    /// Gets the count of [`ping`](PingPong::Ping)s received from the given [`ActorId`].
    ///
    /// Returns [`StateQueryReply::PingCount`].
    PingCount(ActorId),
}

/// The result of successfully processed [`StateQuery`].
///
/// Used in the `state` crate.
#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum StateQueryReply {
    /// Returned from [`StateQuery::Pingers`].
    Pingers(Vec<ActorId>),
    /// Returned from [`StateQuery::PingCount`].
    PingCount(u128),
}
