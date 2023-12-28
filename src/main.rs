use anyhow::Result;
use qdrant_client::prelude::*;
use qdrant_client::qdrant::vectors_config::Config;
use qdrant_client::qdrant::{
    Condition, CreateCollection, Filter, SearchPoints, VectorParams, VectorsConfig,
};
use aleph_alpha_client::{TaskSemanticEmbedding, Client, SemanticRepresentation, Prompt};
use serde_json::json;
use dotenv::dotenv;


#[tokio::main]
async fn main() -> Result<()> {
    // read the api keys from the .env file.
    dotenv().ok();

    // Example of top level client
    // You may also use tonic-generated client from `src/qdrant.rs`
    let client = QdrantClient::from_url("http://localhost:6334").with_api_key("test").build()?;
    

    let collections_list = client.list_collections().await?;
    dbg!(collections_list);


    

    Ok(())
}


fn initialialize_aa_client() -> Client {
    let api_key = std::env::var("ALEPH_ALPHA_API_KEY").unwrap();
    let client = Client::new(&api_key).unwrap();

    return client;
}

fn embedd_text(input: &str) -> Option<()> {

    let client = initialialize_aa_client();

    let task = TaskSemanticEmbedding{Prompt::from_text(input), SemanticRepresentation::Document};

    None
}