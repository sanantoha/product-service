use std::env;
use crate::error::Error;
use mongodb::Client;
use tonic::transport::Server;
use crate::product::ProductService;
use crate::product::proto::product_server::ProductServer;
use crate::product_repository::ProductRepository;

mod error;
mod product_repository;
mod product;
mod models;

const PRODUCT_SERVICE_PORT_NAME: &str = "PRODUCT_PORT";

const PRODUCT_MONGO_URI: &str = "PRODUCT_MONGO_URI";

#[tokio::main]
async fn main() -> Result<(), Error> {
    std::env::set_var("RUST_LOG", std::env::var("RUST_LOG").unwrap_or("info".to_owned()));
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let port = env::var(PRODUCT_SERVICE_PORT_NAME)
        .map_err(|e| Error::Var { input: PRODUCT_SERVICE_PORT_NAME, source: e })?;

    let mongo_uri = env::var(PRODUCT_MONGO_URI)
        .map_err(|e| Error::Var { input: PRODUCT_MONGO_URI, source: e })?;

    let addr = format!("[::1]:{}", port).parse()?;

    let client = Client::with_uri_str(mongo_uri).await?;
    let product_repository = ProductRepository::new(client);
    let product_service = ProductService::new(product_repository);

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(product::proto::FILE_DESCRIPTOR_SET)
        .build_v1().map_err(|e| Error::Generic(e.to_string()))?;

    Server::builder()
        .add_service(reflection_service)
        .add_service(ProductServer::new(product_service))
        .serve(addr)
        .await
        .map_err(|e| Error::Generic(e.to_string()))?;

    Ok(())
}
