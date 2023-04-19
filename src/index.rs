use crate::document::Document;
use crate::embeddings::Embedding;
use serde::{Deserialize, Serialize};
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
    pub async fn new(
        documents: Vec<&Document>,
        config: Option<IndexConfig>,

    ) -> Self {

        let config = config.unwrap_or_default();

        let verbosity = config.verbose;

        let mut vector_index = Self {
            documents: match config.document_chunk_size {
                Some(size) => documents
                    .into_iter()
                    .map(|document| document.clone().chunk(size))
                    .flatten()
                    .collect(),
                None => documents
                    .into_iter()
                    .map(|document| document.clone())
                    .collect(),
            },
            embeddings: None,
            config,
        };

        vector_index._create_embeddings(verbosity).await;
        vector_index
    }

    async fn _create_embeddings(&mut self, verbose: bool) {
        for document in &self.documents {
            let embedding = Embedding::from_document(document).await.unwrap();
            match &mut self.embeddings {
                Some(embeddings) => embeddings.push(embedding),
                None => self.embeddings = Some(vec![embedding]),
            }
        }

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
        let config = config.unwrap_or(&SearchConfig::default());

        let query_embedding =
            Embedding::from_document(&Document::new(query, String::from("query"), Some(0)))
                .await
                .unwrap();

        match &self.embeddings {
            Some(embeddings) => {
                let result: Option<&Embedding> = embeddings.iter().max_by(|a, b| {
                    a.cosine_similarity(&query_embedding)
                        .partial_cmp(&b.cosine_similarity(&query_embedding))
                        .unwrap()
                });

                let source_nodes: Vec<SourceNode> = self
                    .documents
                    .iter()
                    .filter(|document| {
                        document.id == result.unwrap().document_id
                            && document.node_id == result.unwrap().document_node_id
                    })
                    .collect::<Vec<&Document>>()
                    .iter()
                    .map(|document| SourceNode {
                        document,
                        similarity: result.unwrap().cosine_similarity(&query_embedding),
                    })
                    .collect::<Vec<SourceNode>>();

                SearchResult {
                    source_nodes: Some(source_nodes),
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
