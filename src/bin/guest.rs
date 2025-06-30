// use kameo::{
//     actor::RemoteActorRef,
//     remote::{ActorSwarm, dial_opts::DialOpts},
// };
// use kameo_remote_test::{MULTI_ADDR, SomeActor, SomeMessage};
// use log::{debug, error, info};

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     env_logger::Builder::from_default_env()
//         .filter_level(log::LevelFilter::Debug)
//         .init();

//     // debug!("Bootstrapping actor swarm...");
//     // let actor_swarm = ActorSwarm::bootstrap()?;

//     // let dial_opts = DialOpts::unknown_peer_id()
//     //     .address(MULTI_ADDR.parse()?)
//     //     // .address("/ip4/1.2.3.4/tcp/80".parse()?) // ! Sielently fails without error
//     //     .build();

//     // debug!("Dialing actor swarm with options: {:#?}", dial_opts);
//     // actor_swarm.dial(dial_opts).await?;

//     ActorSwarm::bootstrap()?.dial(
//         DialOpts::unknown_peer_id()
//             // .address(MULTI_ADDR.parse()?)
//             .address("/ip4/12.3.0.12/udp/8021/quic-v1".parse()?)
//             .build(),
//     ).await?;

//     tokio::time::sleep(std::time::Duration::from_secs(1)).await;

//     debug!("Spawning SomeActor...");
//     let mb_remote_actor_ref = RemoteActorRef::<SomeActor>::lookup("some_actor").await?;

//     if let Some(remote_actor_ref) = mb_remote_actor_ref {
//         debug!("Remote actor 'some_actor' found, sending message...");

//         let reply = remote_actor_ref.ask(&SomeMessage(21)).await?;

//         info!("Received reply: {}", reply);
//     } else {
//         error!("Remote actor 'some_actor' not found.");
//     }

//     Ok(())
// }

use kameo::{
    actor::RemoteActorRef,
    remote::{ActorSwarm, dial_opts::DialOpts},
};
use kameo_remote_test::{MULTI_ADDR, SomeActor, SomeMessage};
use log::{debug, error, info};
use std::time::{Duration, Instant};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    console_subscriber::init();

    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
    // debug!("Bootstrapping actor swarm...");
    // let actor_swarm = ActorSwarm::bootstrap()?;
    // let dial_opts = DialOpts::unknown_peer_id()
    //     .address(MULTI_ADDR.parse()?)
    //     // .address("/ip4/1.2.3.4/tcp/80".parse()?) // ! Sielently fails without error
    //     .build();
    // debug!("Dialing actor swarm with options: {:#?}", dial_opts);
    // actor_swarm.dial(dial_opts).await?;
    ActorSwarm::bootstrap()?.dial(
        DialOpts::unknown_peer_id()
            .address("/ip4/0.0.0.0/udp/8020/quic-v1".parse()?)
            .build(),
    );
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    debug!("Spawning SomeActor...");
    let mb_remote_actor_ref = RemoteActorRef::<SomeActor>::lookup("some_actor").await?;
    if let Some(remote_actor_ref) = mb_remote_actor_ref {
        debug!("Remote actor 'some_actor' found, sending message...");
        let vector: Vec<u64> = (0..32).collect();
        let start = Instant::now();

        // for idx in 0..10 {
        let mut msg_count: u64 = 0;
        loop {
            // let reply = remote_actor_ref.ask(&SomeMessage(vector.clone())).await?;
            // if reply.len() != vector.len() {
            //     error!("Invalid len()");
            //     break;
            // }
            let reply = remote_actor_ref.tell(&SomeMessage(vector.clone())).await?;
            msg_count += 1;

            if msg_count % 1000 == 0 {
                info!(
                    "Time per message: {:?} us",
                    start.elapsed().as_micros() as f64 / msg_count as f64
                );
            }
            // info!("Received reply: {}", reply);
        }
        // let reply = remote_actor_ref.ask(&SomeMessage(vector.clone())).await?;
        // info!("Elapsed time: {:?}", start.elapsed());
    } else {
        error!("Remote actor 'some_actor' not found.");
    }
    Ok(())
}
