use proto::product_server::Product;
use tonic::{Request, Response, Status};
use crate::product::proto::{Empty, ProductListResponse, ProductRequest, ProductResponse};
use crate::product_repository::ProductRepository;
use log::{info, error};

pub mod proto {
    tonic::include_proto!("product"); // product is a package in product.proto file

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("product_descriptor");
}

#[derive(Debug)]
pub struct ProductService {
    product_repository: ProductRepository
}

impl ProductService {
    pub fn new(repository: ProductRepository) -> Self {
        ProductService { product_repository: repository }
    }
}

#[tonic::async_trait]
impl Product for ProductService {
    async fn save(&self, request: Request<ProductRequest>) -> Result<Response<ProductResponse>, Status> {
        let product_request = request.get_ref();
        info!("Got a save request, for product {:?}", product_request);

        let product = crate::models::Product {
            _id: None,
            name: product_request.name.clone(),
            description: product_request.description.clone(),
            currency: product_request.currency.clone(),
            price: product_request.price,
        };

        let saved_product = self.product_repository.save_product(product).await
            .map_err(|err| {
                error!("Failed to save product {:?}", err);
                Status::internal("could not save product")
            })?;

        let response = ProductResponse {
            id: saved_product._id.map(|id| id.to_hex()).unwrap_or_default(),
            name: saved_product.name,
            description: saved_product.description,
            current: saved_product.currency,
            price: saved_product.price,
        };

        Ok(Response::new(response))
    }

    async fn get_product_list(&self, _: Request<Empty>) -> Result<Response<ProductListResponse>, Status> {
        info!("Got a get product list request");

        let products = self.product_repository.get_product_list().await
            .map_err(|err| {
                error!("Failed to get list of products {:?}", err);
                Status::internal("could not get list of products")
        })?;

        info!("Got products {} from database", products.len());

        let mut product_responses = vec![];

        for product in products {
            product_responses.push(ProductResponse {
                id: product._id.map(|id| id.to_hex()).unwrap_or_default(),
                name: product.name,
                description: product.description,
                current: product.currency,
                price: product.price,
            });
        }

        let response = ProductListResponse {
            products: product_responses
        };

        Ok(Response::new(response))
    }
}