mod index;
mod embeddings;
mod document;
use tokio;

use index::{VectorIndex, IndexConfig};
use document::{Document};


#[tokio::main]
async fn main() {
    let document = Document::from_file("test.txt");

    let config = IndexConfig {
        document_chunk_size: Some(100),
        verbose: true,
    };

    let mut index = VectorIndex::new(vec![&document], Some(config)).await;

    index.search("Some seach query".to_string(), None).await;

}
