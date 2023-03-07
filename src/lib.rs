/// Returns the hash of the uploaded content
pub async fn upload_to_ipfs(
    server_address: std::net::SocketAddr,
    data: impl std::io::Read + Send + Sync + Unpin + 'static,
) -> Result<String, IpfsUploadError> {
    use ipfs_api_backend_hyper::{IpfsApi as _, IpfsClient, TryFromUri as _};

    let client = IpfsClient::from_socket(
        // BUG?(aatifsyed): using HTTP panics:
        //     Client(hyper::Error(Connect, "invalid URL, scheme is not http"))
        http::uri::Scheme::HTTP,
        server_address,
    )
    .map_err(IpfsUploadError::ClientCreationError)?;

    let added = client
        .add(data)
        .await
        .map_err(IpfsUploadError::UploadError)?;

    Ok(added.hash)
}

#[derive(Debug, thiserror::Error, enum_as_inner::EnumAsInner)]
pub enum IpfsUploadError {
    #[error("failed to create ipfs client")]
    ClientCreationError(#[source] http::Error),
    #[error("failed to upload data")]
    UploadError(#[source] ipfs_api_backend_hyper::Error),
}
