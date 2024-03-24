use anyhow::{Result};
use qdrant_client::prelude::*;
use qdrant_client::qdrant::{
    CreateCollection, PointStruct, SearchPoints, Condition, Filter, VectorParams, VectorsConfig
};
use qdrant_client::qdrant::vectors_config::Config;
use serde_json::json;
use std::convert::TryInto;

#[tokio::main]
async fn main() -> Result<()> {
    let config = QdrantClientConfig::from_url("http://localhost:6334");
    let client = QdrantClient::new(Some(config))?;

    let collection_name = "test_collection";

    // Inline create_collection
    let _ = client.delete_collection(collection_name).await;
    client.create_collection(&CreateCollection {
        collection_name: collection_name.into(),
        vectors_config: Some(VectorsConfig {
            config: Some(Config::Params(VectorParams {
                size: 4,
                distance: Distance::Cosine.into(),
                ..Default::default()
            })),
        }),
        ..Default::default()
    }).await?;

    // Inline ingest_vectors
    let points = vec![
        PointStruct::new(
            1,
            vec![0.05, 0.61, 0.76, 0.74],
            json!({"city": "Berlin"}).try_into().unwrap(),
        ),
        PointStruct::new(
            2,
            vec![0.19, 0.81, 0.75, 0.11],
            json!({"city": "London"}).try_into().unwrap(),
        ),
        PointStruct::new(
            3,
            vec![0.12, 0.72, 0.69, 0.33],
            json!({"city": "Paris"}).try_into().unwrap(),
        ),
        PointStruct::new(
            4,
            vec![0.42, 0.21, 0.54, 0.85],
            json!({"city": "New York"}).try_into().unwrap(),
        ),
        PointStruct::new(
            5,
            vec![0.91, 0.62, 0.34, 0.25],
            json!({"city": "Tokyo"}).try_into().unwrap(),
        ),
        PointStruct::new(
            6,
            vec![0.34, 0.81, 0.91, 0.43],
            json!({"city": "Sydney"}).try_into().unwrap(),
        ),
        PointStruct::new(
            7,
            vec![0.76, 0.11, 0.27, 0.64],
            json!({"city": "Moscow"}).try_into().unwrap(),
        ),
        PointStruct::new(
            8,
            vec![0.28, 0.39, 0.15, 0.79],
            json!({"city": "Beijing"}).try_into().unwrap(),
        ),
        PointStruct::new(
            9,
            vec![0.67, 0.44, 0.52, 0.18],
            json!({"city": "Rio de Janeiro"}).try_into().unwrap(),
        ),
        PointStruct::new(
            10,
            vec![0.56, 0.73, 0.94, 0.27],
            json!({"city": "Cape Town"}).try_into().unwrap(),
        ),
        // Adding more points
        PointStruct::new(
            11,
            vec![0.33, 0.98, 0.54, 0.12],
            json!({"city": "Mumbai"}).try_into().unwrap(),
        ),
        PointStruct::new(
            12,
            vec![0.88, 0.44, 0.31, 0.67],
            json!({"city": "Los Angeles"}).try_into().unwrap(),
        ),
        PointStruct::new(
            13,
            vec![0.29, 0.55, 0.71, 0.22],
            json!({"city": "Chicago"}).try_into().unwrap(),
        ),
        PointStruct::new(
            14,
            vec![0.45, 0.79, 0.88, 0.33],
            json!({"city": "Toronto"}).try_into().unwrap(),
        ),
        PointStruct::new(
            15,
            vec![0.65, 0.30, 0.48, 0.91],
            json!({"city": "Vancouver"}).try_into().unwrap(),
        )
    ];
    client.upsert_points_blocking(collection_name, None, points, None).await?;

    // Inline query_vectors
    let search_result = client.search_points(&SearchPoints {
        collection_name: collection_name.into(),
        vector: vec![0.2, 0.1, 0.9, 0.7],
        limit: 3,
        with_payload: Some(true.into()),
        ..Default::default()
    }).await?;
    println!("Search Results:");
    for (index, point) in search_result.result.iter().enumerate() {
        let formatted_payload = serde_json::to_string_pretty(&point.payload).unwrap();
        println!("Point {}: ", index + 1);
        println!(" - Payload: {}", formatted_payload);
        println!(" - Score: {}", point.score);
        println!();
    }

    // Inline query_vectors_with_filter
    let search_result_filtered = client.search_points(&SearchPoints {
        collection_name: collection_name.into(),
        vector: vec![0.2, 0.1, 0.9, 0.7],
        filter: Some(Filter::all([Condition::matches("city", "London".to_string())])),
        limit: 2,
        with_payload: Some(true.into()),
        ..Default::default()
    }).await?;
    println!("Search Results After Filter:");
    for (index, point) in search_result_filtered.result.iter().enumerate() {
        let formatted_payload = serde_json::to_string_pretty(&point.payload).unwrap();
        println!("Point {}: ", index + 1);
        println!(" - Payload: {}", formatted_payload);
        println!(" - Score: {}", point.score);
        println!();
    }

    Ok(())
}
