# Endex

### Usage
```rust
use endex::{VectorIndex, IndexConfig, Document};


#[tokio::main]
async fn main() {
    let document = Document::from_file("test.txt");
    let document2 = Document::new("id".to_string(), "content".to_string());

    let config = IndexConfig {
        document_chunk_size: Some(100),
        verbose: true,
    };

    let mut index = VectorIndex::new(vec![&document], Some(config)).await;

    index.search("Some seach query".to_string(), None).await;
}

```
