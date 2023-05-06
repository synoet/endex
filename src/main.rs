mod document;
mod embeddings;
mod index;

use document::Document;
use index::{IndexConfig, VectorIndex};

#[tokio::main]
async fn main() {
    let document_a = Document::new(
        "The quick brown fox jumped over the lazy river".to_string(),
        "a".to_string(),
    );
    let document_b = Document::new(
        "The Jedi fights a battle with a monster".to_string(),
        "b".to_string(),
    );

    let config = IndexConfig {
        document_chunk_size: None,
        verbose: true,
    };

    let mut index = VectorIndex::new(vec![&document_a, &document_b], Some(config)).await;

    let result = index.search("war".to_string(), None).await;

    println!("{:?}", result);
}
