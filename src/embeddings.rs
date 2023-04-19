use crate::document::Document;
use itertools::concat;
use ndarray::Array1;
use serde_json;
use dotenv;

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
struct Response {
    data: Vec<EmbeddingObject>,
    model: String,
    object: String,
    usage: Usage,
}

#[derive(Serialize)]
struct Request {
    model: String,
    input: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Embedding {
    pub data: Vec<Vec<f64>>,
    pub document_id: String,
    pub document_node_id: usize,
    pub tokens_used: f64,
}

impl Embedding {
    pub async fn from_document(document: &Document) -> Result<Embedding, reqwest::Error> {
        let client = reqwest::Client::new();

        let payload = Request {
            model: String::from("text-embedding-ada-002"),
            input: document.text.clone(),
        };

        let response = client
            .post("https://api.openai.com/v1/embeddings")
            .body(serde_json::to_string(&payload).unwrap())
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                "Bearer ".to_owned() +  &dotenv::var("OPENAI_API_KEY").unwrap(),
            )
            .send()
            .await?
            .json::<Response>()
            .await?;

        let embedding: Embedding = Embedding {
            data: response
                .data
                .iter()
                .map(|obj| obj.embedding.clone())
                .collect(),
            document_id: document.id.clone(),
            document_node_id: document.node_id,
            tokens_used: response.usage.total_tokens,
        };

        Ok(embedding)
    }

    pub fn cosine_similarity(&self, other: &Self) -> f64 {
        let curr = Array1::from_vec(concat(self.data.clone()));
        let other = Array1::from_vec(concat(other.data.clone()));

        curr.dot(&other) / (curr.mapv(|x| x * x).sum().sqrt() * other.mapv(|x| x * x).sum().sqrt())
    }
}
