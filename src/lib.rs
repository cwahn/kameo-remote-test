use kameo::{
    Actor, RemoteActor,
    prelude::{Context, Message},
    remote_message,
};
use serde::{Deserialize, Serialize};

pub const MULTI_ADDR: &str = "/ip4/0.0.0.0/tcp/8020";

#[derive(Actor, RemoteActor)]
pub struct SomeActor {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SomeMessage(pub u64);

#[remote_message("9c895a11-cb75-4102-8b6f-0aa36c70b0c1")]
impl Message<SomeMessage> for SomeActor {
    type Reply = u64;

    async fn handle(
        &mut self,
        msg: SomeMessage,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        log::info!("Received message: {:?}", msg);
        msg.0 * 2
    }
}
