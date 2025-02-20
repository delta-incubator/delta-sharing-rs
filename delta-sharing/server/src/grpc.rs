use delta_sharing_common::models::sharing::v1::delta_sharing_service_server::DeltaSharingServiceServer;
use delta_sharing_common::{DeltaSharingHandler, Error, Result};
use tonic::transport::Server;

// TODO(roeap)
// - make auth configurable
// - make log level configurable

pub async fn run_server(
    host: impl AsRef<str>,
    port: u16,
    handler: DeltaSharingHandler,
) -> Result<()> {
    let addr = format!("{}:{}", host.as_ref(), port)
        .parse()
        .map_err(|_| Error::generic("Invalid address."))?;

    tracing::info!("Listning on: {addr}");

    Server::builder()
        .trace_fn(|_| tracing::info_span!("delta_sharing_server"))
        .add_service(DeltaSharingServiceServer::new(handler))
        .serve(addr)
        .await
        .map_err(|e| Error::Generic(e.to_string()))?;

    Ok(())
}
