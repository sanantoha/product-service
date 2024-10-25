use proto::product_server::Product;
use tonic::{Request, Response, Status};
use crate::product::proto::{Empty, ProductListResponse, ProductRequest, ProductResponse, DeleteProductRequest, DeleteProductResponse};
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

        let product_responses: Vec<ProductResponse> = products.into_iter().map(|product| {
            ProductResponse {
                id: product._id.map(|id| id.to_hex()).unwrap_or_default(),
                name: product.name,
                description: product.description,
                current: product.currency,
                price: product.price,
            }
        }).collect();

        let response = ProductListResponse {
            products: product_responses
        };

        Ok(Response::new(response))
    }

    async fn delete_product(&self, request: Request<DeleteProductRequest>) -> Result<Response<DeleteProductResponse>, Status> {
        let delete_product_request = request.get_ref();
        info!("Got delete product request: {:?}", delete_product_request);

        let id = self.product_repository.delete_product(&delete_product_request.id).await
            .map_err(|err| {
                error!("Failed to delete product id: {} {:?}", delete_product_request.id, err);
                Status::internal(format!("could not delete product id {}", delete_product_request.id))
            })?;

        let mut deleted_msg = "is not deleted";
        if id {
            deleted_msg = "is deleted";
        }
        info!("product id {} {}", delete_product_request.id, deleted_msg);
        Ok(Response::new(DeleteProductResponse { is_deleted: id }))
    }
}