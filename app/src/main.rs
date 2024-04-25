use gstd::{ActorId, Encode};
use template_io::StateQuery;

fn main() {
    let state_query = StateQuery::PingCount(ActorId::zero());
    println!("{:?}", state_query.encode());
}
