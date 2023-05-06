use crate::document::{Document, DocumentNode};
use crate::embeddings::{Embedding, EmbeddingResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};

#[derive(Debug, Serialize, Deserialize)]
pub struct VectorIndex {
    pub documents: Vec<Document>,
    embeddings: Option<Vec<Embedding>>,
    config: IndexConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexConfig {
    pub document_chunk_size: Option<usize>,
    pub verbose: bool,
}

impl Default for IndexConfig {
    fn default() -> Self {
        Self {
            document_chunk_size: None,
            verbose: false,
        }
    }
}

#[derive(Debug)]
pub struct SourceNode<'a> {
    pub document: &'a Document,
    pub document_node: &'a DocumentNode,
    pub similarity: f64,
}

#[derive(Clone)]
pub struct SearchConfig {
    pub top_k: Option<usize>,
    pub threshold: Option<f32>,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            top_k: Some(1),
            threshold: Some(0.0),
        }
    }
}

#[derive(Debug)]
pub struct SearchResult<'a> {
    pub source_nodes: Option<Vec<SourceNode<'a>>>,
}

impl VectorIndex {
    pub async fn new(documents: Vec<&Document>, config: Option<IndexConfig>) -> Self {
        let config = config.unwrap_or_default();

        let verbosity = config.verbose;

        let mut vector_index = Self {
            documents: documents
                .into_iter()
                .map(|document| {
                    let mut temp_document = document.clone();
                    let chunk_size = match config.document_chunk_size {
                        Some(size) => size,
                        None => document.text.len(),
                    };
                    temp_document.chunk(chunk_size);
                    temp_document
                })
                .collect::<Vec<Document>>(),
            embeddings: None,
            config,
        };

        vector_index._create_embeddings(verbosity).await;
        vector_index
    }

    async fn _create_embeddings(&mut self, verbose: bool) {
        let mut all_embeddings: Vec<Embedding> = Vec::new();

        for document in &self.documents {
            let embeddings = Embedding::from_document(document).await.unwrap();

            match embeddings {
                EmbeddingResult::Multiple(embeddings) => {
                    all_embeddings.extend(embeddings);
                }
                EmbeddingResult::Single(embedding) => {
                    all_embeddings.push(embedding);
                }
            }
        }

        self.embeddings = Some(all_embeddings);

        if verbose {
            println!("Created embeddings for {} documents", self.documents.len());
            println!(
                "Total number of tokens used: {}",
                self.embeddings
                    .as_ref()
                    .unwrap()
                    .iter()
                    .fold(0.0, |acc, embedding| acc + embedding.tokens_used)
            );
        }
    }

    pub async fn search(&mut self, query: String, config: Option<&SearchConfig>) -> SearchResult {
        let _config = config.unwrap_or(&SearchConfig::default());

        let query_embedding = Embedding::from_query(&query).await.unwrap();

        match &self.embeddings {
            Some(embeddings) => {
                let document_map: HashMap<String, &Document> = self
                    .documents
                    .iter()
                    .map(|document| (document.id.clone(), document))
                    .collect();

                let mut results: Vec<SourceNode> = vec![];

                for embedding in embeddings {
                    let document = document_map.get(&embedding.document_id).unwrap();

                    let node = document
                        .nodes
                        .as_ref()
                        .unwrap()
                        .iter()
                        .find(|node| node.id == embedding.document_node_id);

                    let similarity = embedding.cosine_similarity(&query_embedding);

                    results.push(SourceNode {
                        document,
                        document_node: node.unwrap(),
                        similarity,
                    });
                }

                results.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());

                SearchResult {
                    source_nodes: Some(results),
                }
            }
            None => SearchResult { source_nodes: None },
        }
    }

    pub fn save_to_disk(&self, path: &str) -> Result<(), std::io::Error> {
        let file = File::create(path);
        let writer = BufWriter::new(file.unwrap());
        serde_json::to_writer_pretty(writer, &self)?;
        Ok(())
    }

    pub fn load_from_disk(path: &str) -> Result<Self, std::io::Error> {
        let file = File::open(path);
        let reader = BufReader::new(file.unwrap());
        let vector_index: Self = serde_json::from_reader(reader)?;
        Ok(vector_index)
    }
}
