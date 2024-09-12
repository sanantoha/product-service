use crate::models::Product;
use mongodb::{bson::Bson, bson::doc, Client, Collection};
use log::info;
use tonic::codegen::tokio_stream::StreamExt;
use crate::error::Error;

#[derive(Debug)]
pub struct ProductRepository {
    collection: Collection<Product>,
}

impl ProductRepository {
    pub fn new(client: Client) -> Self {
        let database = client.database("product-service");
        let collection = database.collection::<Product>("products");

        ProductRepository{ collection }
    }

    pub async fn save_product(&self, mut product: Product) -> Result<Product, Error> {
        let insert_res = self.collection.insert_one(&product).await?;

        match insert_res.inserted_id {
            Bson::ObjectId(oid) => {
                // let id = oid.to_hex();
                info!("inserted product _id: {}", oid);
                product._id = Some(oid);
                Ok(product)
            },
            _ => Err(Error::MongoKey(format!("for product {}", product.name))),
        }
    }

    pub async fn get_product_list(&self) -> Result<Vec<Product>, Error> {
        let mut cursor = self.collection.find(doc! {}).await?;

        let mut products: Vec<Product> = vec![];

        while let Some(product) = cursor.next().await {
            products.push(product?);
        }

        Ok(products)
    }
}
