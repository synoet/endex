use crate::document::{Document, DocumentNode};
use dotenv;
use itertools::concat;
use ndarray::Array1;
use serde_json;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Usage {
    prompt_tokens: f64,
    total_tokens: f64,
}

#[derive(Deserialize)]
struct EmbeddingObject {
    embedding: Vec<f64>,
    index: i64,
    object: String,
}

#[derive(Deserialize)]
struct GPTEmbeddingResponse {
    data: Vec<EmbeddingObject>,
    model: String,
    object: String,
    usage: Usage,
}

impl GPTEmbeddingResponse {
    pub fn to_embedding_format(&self) -> Vec<Vec<f64>> {
        self.data
            .iter()
            .map(|obj| obj.embedding.clone())
            .collect::<Vec<Vec<f64>>>()
    }
}

#[derive(Serialize)]
pub struct GPTEmbeddingRequest {
    pub model: String,
    pub input: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Embedding {
    pub data: Vec<Vec<f64>>,
    pub tokens_used: f64,
    pub document_id: String,
    pub document_node_id: String,
}

pub enum EmbeddingResult {
    Single(Embedding),
    Multiple(Vec<Embedding>),
}

impl Embedding {
    pub async fn create_gpt_embedding(
        request: GPTEmbeddingRequest,
    ) -> Result<GPTEmbeddingResponse, reqwest::Error> {
        let client = reqwest::Client::new();
        let response = client
            .post("https://api.openai.com/v1/embeddings")
            .body(serde_json::to_string(&request).unwrap())
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                "Bearer ".to_owned() + &dotenv::var("OPENAI_API_KEY").unwrap(),
            )
            .send()
            .await?
            .json::<GPTEmbeddingResponse>()
            .await?;

        Ok(response)
    }

    pub async fn from_document_node(
        document: &Document,
        document_node: &DocumentNode,
    ) -> Result<Embedding, reqwest::Error> {
        let payload = GPTEmbeddingRequest {
            model: String::from("text-embedding-ada-002"),
            input: document_node.text.clone(),
        };

        let response: GPTEmbeddingResponse = Embedding::create_gpt_embedding(payload).await?;

        Ok(Embedding {
            data: response.to_embedding_format(),
            document_id: document.id.clone(),
            document_node_id: document_node.id.clone(),
            tokens_used: response.usage.total_tokens,
        })
    }

    /// Creates an embedding from a query string
    pub async fn from_query(query: &str) -> Result<Embedding, reqwest::Error> {
        let payload = GPTEmbeddingRequest {
            model: "text-embedding-ada-002".to_string(),
            input: query.to_string(),
        };

        let response: GPTEmbeddingResponse = Embedding::create_gpt_embedding(payload).await?;

        Ok(Embedding {
            data: response.to_embedding_format(),
            document_id: String::from("query"),
            document_node_id: String::from("query"),
            tokens_used: response.usage.total_tokens,
        })
    }

    /// Creates an EmbeddingResult from a given document
    /// if `document.nodes.len() > 1` -> `EmbeddingResult::Multiple` (Multiple Embeddings)
    /// if `document.nodes.len() == 1` -> `EmbeddingResult::Single` (A Single Embedding)
    pub async fn from_document(document: &Document) -> Result<EmbeddingResult, reqwest::Error> {
        let mut embeddings: Vec<Embedding> = vec![];

        match &document.nodes {
            Some(nodes) => {
                for node in nodes.iter() {
                    embeddings.push(Embedding::from_document_node(document, &node).await?);
                }
            }
            None => {
                let embedding = Embedding::from_document_node(
                    document,
                    &DocumentNode {
                        id: String::from("0"),
                        text: document.text.clone(),
                        document_id: document.id.clone(),
                    },
                )
                .await?;

                return Ok(EmbeddingResult::Single(embedding));
            }
        }

        Ok(EmbeddingResult::Multiple(embeddings))
    }

    pub fn cosine_similarity(&self, other: &Self) -> f64 {
        let curr = Array1::from_vec(concat(self.data.clone()));
        let other = Array1::from_vec(concat(other.data.clone()));

        curr.dot(&other) / (curr.mapv(|x| x * x).sum().sqrt() * other.mapv(|x| x * x).sum().sqrt())
    }
}
