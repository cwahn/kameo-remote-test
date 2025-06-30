use kameo::{
    Actor,
    remote::{ActorSwarm, ActorSwarmBehaviour},
};
use kameo_remote_test::{MULTI_ADDR, SomeActor};
use libp2p::{
    StreamProtocol, SwarmBuilder,
    kad::{self, store::MemoryStore},
    mdns,
    request_response::{self, ProtocolSupport},
};
use log::debug;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    debug!("Bootstrapping actor swarm...");

    // // Per https://github.com/tqwewe/kameo/issues/88
    ActorSwarm::bootstrap_with_swarm(
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
    .listen_on("/ip4/0.0.0.0/tcp/8020".parse()?)
    .await?;

    debug!("Spawning SomeActor...");
    let some_actor_ref = SomeActor::spawn(SomeActor {
        init_time: std::time::Instant::now(),
        count: 0,
    });

    debug!("Registering SomeActor with name 'some_actor'...");
    some_actor_ref.register("some_actor").await?;

    debug!("Actor swarm is running. Press Ctrl+C to exit.");
    tokio::signal::ctrl_c().await?;

    Ok(())
}
