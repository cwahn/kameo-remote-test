use std::time::Instant;

use kameo::{
    actor::RemoteActorRef,
    remote::{ActorSwarm, ActorSwarmBehaviour},
};
use kameo_remote_test::{MULTI_ADDR, SomeActor, SomeMessage};
use libp2p::{
    StreamProtocol, SwarmBuilder,
    kad::{self, store::MemoryStore},
    mdns,
    request_response::{self, ProtocolSupport},
};
use log::{debug, error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    console_subscriber::init();

    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    let _listener_id = ActorSwarm::bootstrap_with_swarm(
        SwarmBuilder::with_new_identity()
            .with_tokio()
            .with_tcp(
                Default::default(),
                libp2p_noise::Config::new,
                libp2p_yamux::Config::default,
            )?
            .with_behaviour(|keypair| {
                Ok(ActorSwarmBehaviour {
                    kademlia: kad::Behaviour::new(
                        keypair.public().to_peer_id(),
                        MemoryStore::new(keypair.public().to_peer_id()),
                    ),
                    mdns: mdns::tokio::Behaviour::new(
                        mdns::Config::default(),
                        keypair.public().to_peer_id(),
                    )?,
                    request_response: request_response::cbor::Behaviour::new(
                        [(StreamProtocol::new("/kameo/1"), ProtocolSupport::Full)],
                        request_response::Config::default(),
                    ),
                })
            })?
            .build(),
    )?
    .listen_on("/ip4/127.0.0.1/tcp/8020".parse()?)
    .await?;

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    debug!("Spawning SomeActor...");
    let mb_remote_actor_ref = RemoteActorRef::<SomeActor>::lookup("some_actor").await?;
    if let Some(remote_actor_ref) = mb_remote_actor_ref {
        debug!("Remote actor 'some_actor' found, sending message...");
        let vector: Vec<u64> = (0..32).collect();
        let start = Instant::now();

        let mut msg_count: u64 = 0;
        loop {
            remote_actor_ref.tell(&SomeMessage(vector.clone())).await?;
            msg_count += 1;

            if msg_count % 1000 == 0 {
                info!(
                    "Time per message: {:?} us",
                    start.elapsed().as_micros() as f64 / msg_count as f64
                );
            }
        }
    } else {
        error!("Remote actor 'some_actor' not found.");
    }
    Ok(())
}
