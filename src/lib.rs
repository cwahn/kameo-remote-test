// use kameo::{
//     Actor, RemoteActor,
//     prelude::{Context, Message},
//     remote_message,
// };
// use serde::{Deserialize, Serialize};

// // pub const MULTI_ADDR: &str = "/ip4/0.0.0.0/tcp/8020";
// // pub const MULTI_ADDR: &str = "/ip4/0.0.0.0/udp/8020/quic-v1";
// // pub const MULTI_ADDR: &str = "/ip4/0.0.0.0/udp/8020/quic-v1"; // This address works.
// pub const MULTI_ADDR: &str = "/ip4/0.0.0.0/tcp/8020";

// #[derive(Actor, RemoteActor)]
// pub struct SomeActor {}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SomeMessage(pub u64);

// #[remote_message("9c895a11-cb75-4102-8b6f-0aa36c70b0c1")]
// impl Message<SomeMessage> for SomeActor {
//     type Reply = u64;

//     async fn handle(
//         &mut self,
//         msg: SomeMessage,
//         _ctx: &mut Context<Self, Self::Reply>,
//     ) -> Self::Reply {
//         log::info!("Received message: {:?}", msg);
//         msg.0 * 2
//     }
// }

use std::sync::OnceLock;

use kameo::{
    Actor, RemoteActor,
    prelude::{Context, Message},
    remote_message,
};

use serde::{Deserialize, Serialize};

// pub const MULTI_ADDR: &str = "/ip4/0.0.0.0/tcp/8020";
pub const MULTI_ADDR: &str = "/ip4/0.0.0.0/udp/8020/quic-v1";

// pub const INIT_TIMEPOINT: &std::time::Instant = &std::time::Instant::now();

#[derive(Actor, RemoteActor)]
pub struct SomeActor {
    pub init_time: std::time::Instant,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SomeMessage(pub Vec<u64>);
#[remote_message("9c895a11-cb75-4102-8b6f-0aa36c70b0c1")]
impl Message<SomeMessage> for SomeActor {
    // type Reply = Vec<u64>;
    type Reply = ();

    async fn handle(
        &mut self,
        _msg: SomeMessage,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        // log::info!("Received message: {:?}", msg);
        // msg.0
        self.count += 1;
        if self.count % 1000 == 0 {
            log::info!(
                "Time per message: {:?} us",
                self.init_time.elapsed().as_micros() as f64 / self.count as f64
            );
        }
    }
}
