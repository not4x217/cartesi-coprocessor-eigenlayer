use std::env;

use cartesi_coprocessor_evm::gio_client::GIOClient;
use json::{object, JsonValue};
use url::Url;

use alloy_primitives::{hex, U256, BlockHash, Address};
use alloy_sol_types::{sol, SolType};

mod eigenlayer;

pub async fn handle_advance(
    client: &hyper::Client<hyper::client::HttpConnector>,
    server_addr: &str,
    request: JsonValue,
) -> Result<&'static str, Box<dyn std::error::Error>> {
    println!("Received advance request data {}", &request);
    let payload = request["data"]["payload"]
        .as_str()
        .ok_or("Missing payload")?;

    let payload_data = hex::decode(payload)?;

    type PayloadValues = sol! { tuple(bytes32, address, address) };
    let payload_values = PayloadValues::abi_decode(&payload_data)?;

    // !!!
    println!("block hash: {}", BlockHash::from(payload_values.0));
    println!("operator: {}", payload_values.1);
    println!("erc20: {}", payload_values.2);

    let gio_url_base = Url::parse(server_addr)?;
    let gio_url = gio_url_base.join("gio")?;

    // !!!
    println!("gio url: {}", gio_url);

    let gio_client = GIOClient::new(gio_url);

    let block_hash = BlockHash::from(payload_values.0);
    let operator = payload_values.1;
    let erc_20 = payload_values.2;  
     
    let operator_balance = eigenlayer::query_operator_token_balance(
        gio_client,
        BlockHash::from(payload_values.0),
        operator,
        erc_20,
    ).await?;

    // !!!
    println!("operator balance: {}", operator_balance);

    emit_abi_encoded_notice(
        client,
        server_addr,
        block_hash,
        operator,
        erc_20,
        operator_balance
    ).await?;
    
    Ok("accept")
}

async fn emit_abi_encoded_notice(
    client: &hyper::Client<hyper::client::HttpConnector>,
    server_addr: &str,
    block_hash: BlockHash,
    operator: Address,
    erc_20: Address,
    operator_balance: U256,
) -> Result<(), Box<dyn std::error::Error>> {
    // Encode payload
    type NoticeValues = sol! { tuple(bytes32, address, address, uint256) };
    let notice_values = (block_hash, operator, erc_20, operator_balance);
    let notice_data = NoticeValues::abi_encode(&notice_values);

    // Create and emit notice
    let notice_hex = hex::encode_prefixed(&notice_data);
    let notice = object! { "payload" => notice_hex };
    let notice_request = hyper::Request::builder()
        .method(hyper::Method::POST)
        .uri(format!("{}/notice", server_addr))
        .header("Content-Type", "application/json")
        .body(hyper::Body::from(notice.dump()))?;

    let _response = client.request(notice_request).await?;
    println!("ABI encoded notice emitted successfully");
    Ok(())
}

pub async fn handle_inspect(
    _client: &hyper::Client<hyper::client::HttpConnector>,
    _server_addr: &str,
    request: JsonValue,
) -> Result<&'static str, Box<dyn std::error::Error>> {
    println!("Received inspect request data {}", &request);
    let _payload = request["data"]["payload"]
        .as_str()
        .ok_or("Missing payload")?;
    // TODO: add application logic here
    Ok("accept")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = hyper::Client::new();
    let server_addr = env::var("ROLLUP_HTTP_SERVER_URL")?;

    let mut status = "accept";
    loop {
        println!("Sending finish");
        let response = object! {"status" => status.clone()};
        let request = hyper::Request::builder()
            .method(hyper::Method::POST)
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .uri(format!("{}/finish", &server_addr))
            .body(hyper::Body::from(response.dump()))?;
        let response = client.request(request).await?;
        println!("Received finish status {}", response.status());

        if response.status() == hyper::StatusCode::ACCEPTED {
            println!("No pending rollup request, trying again");
        } else {
            let body = hyper::body::to_bytes(response).await?;
            let utf = std::str::from_utf8(&body)?;
            let req = json::parse(utf)?;

            let request_type = req["request_type"]
                .as_str()
                .ok_or("request_type is not a string")?;
            status = match request_type {
                "advance_state" => handle_advance(&client, &server_addr[..], req).await?,
                "inspect_state" => handle_inspect(&client, &server_addr[..], req).await?,
                &_ => {
                    eprintln!("Unknown request type");
                    "reject"
                }
            };
        }
    }
}
