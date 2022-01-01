use serde::{Deserialize, Serialize};

#[srpc::rpc]
#[derive(Default)]
pub struct PingServer;

#[derive(Serialize, Deserialize)]
pub struct Ping(pub u32);
#[derive(Serialize, Deserialize)]
pub struct Pong(pub u32);

#[srpc::rpc]
impl PingServer {
    pub async fn ping(&self, ping: Ping) -> Pong {
        let Ping(ping) = ping;
        Pong(ping)
    }
}
