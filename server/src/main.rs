use canary::Addr;
use server::PingServer;
use srpc::canary::routes::GLOBAL_ROUTE;

#[canary::main]
async fn main() -> canary::Result<()> {
    GLOBAL_ROUTE.add_service_at::<PingServer>("ping", Default::default())?;
    let addr = "wss@127.0.0.1:8085".parse::<Addr>()?;
    addr.bind().await?;
    std::future::pending().await
}
