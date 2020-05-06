use dgraph_grpc_client::api::*;

use std::collections::HashMap;
use std::string::String;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut client = dgraph_client::DgraphClient::connect("http://localhost:9080").await?;

  let response = client
    .query(tonic::Request::new(Request {
      query: String::from("{ node(func: uid(0x1)) { uid dgraph.type } }"),
      best_effort: true,
      read_only: true,
      commit_now: true,
      start_ts: 0,
      vars: HashMap::new(),
      mutations: Vec::new(),
    }))
    .await?;

  println!("response = {:?}", String::from_utf8(response.into_inner().json).unwrap());

  Ok(())
}
