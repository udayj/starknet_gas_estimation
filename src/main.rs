use avnu_starknet::client::Client;
use avnu_starknet::types::{to_short_string, StarknetByteArray};
use avnu_starknet::{no_calldata, Address, BlockNumber, Error};
use avnu_starknet::{DataAvailabilityMode, ResourceBounds, ResourceBoundsMapping};
use avnu_starknet::{
    ExecutableInvokeTransaction, ExecutableTransaction, InvokeTransaction, InvokeTransactionV1,
    InvokeTransactionV3, ExecutableInvokeTransactionV3
};
use csv::Writer;
use serde::{Deserialize, Serialize};
use starknet_core::types::FieldElement;
use std::{fs::File, io::Write, str::FromStr};
use avnu_starknet::{storage_key, storage_mapping_key};

#[derive(Debug, Deserialize)]
struct SimulationInput {
    max_fee: String,
    token_from: String,
    token_from_low: String,
    token_to: String,
    token_to_min_low: String,
    price_distance: String,
    ticks_crossed: String,
}

#[derive(Debug, Serialize)]
struct SimulationResult {
    token_from: String,
    token_to: String,
    from_amount: String,
    data_gas_consumed: String,
    gas_consumed: String,
    gas_price: String,
    overall_fee: String,
    ticks_crossed: String,
}

fn client() -> Client {
    //Client::new("https://sepolia.juno.avnu.fi")
    Client::new("https://sepolia.juno.avnu.fi/v0_7")
}

async fn get_nonce(
    account_address: &FieldElement,
) -> Result<FieldElement, Box<dyn std::error::Error>> {
    let client = client();
    let nonce = client.get_nonce(account_address.clone()).await?;
    Ok(nonce)
}

async fn simulate_transaction(
    input: &SimulationInput,
    account_address: &FieldElement,
    nonce: &FieldElement,
) -> Result<SimulationResult, Box<dyn std::error::Error>> {
    // Get the current nonce
    println!("Simulation for TX V1");
    let client = client();

    let tx1 = InvokeTransactionV1 {
        transaction_hash: FieldElement::from_str("0x0")?,
        nonce: *nonce,
        max_fee: FieldElement::from_str(&input.max_fee)?,
        sender_address: *account_address,
        signature: Vec::new(),
        calldata: vec![
            FieldElement::from_str("0x1")?, // no. of calls
            FieldElement::from_str(
                "0x02c56e8b00dbe2a71e57472685378fc8988bba947e9a99b26a00fade2b4fe7c2",
            )?, // contract_address
            FieldElement::from_str(
                "0x01171593aa5bdadda4d6b0efde6cc94ee7649c3163d5efeb19da6c16d63a2a63",
            )?, // function selector
            FieldElement::from_str("0x17")?, // size of arguments
            FieldElement::from_str(&input.token_from)?, // token_from
            FieldElement::from_str(&input.token_from_low)?, // token from low
            FieldElement::from_str("0x0")?, // token from high
            FieldElement::from_str(&input.token_to)?, // token_to_address
            FieldElement::from_str("0x0")?, // token to low
            FieldElement::from_str("0x0")?, // token to high
            FieldElement::from_str(&input.token_to_min_low)?, // token to min low
            FieldElement::from_str("0x0")?, // token to min high
            *account_address,               // beneficiary
            FieldElement::from_str("0x0")?, // integrator fee
            *account_address,               // integrator recipient
            FieldElement::from_str("0x1")?, // no. of routes
            FieldElement::from_str(&input.token_from)?, // sell_token
            FieldElement::from_str(&input.token_to)?, // buy token
            FieldElement::from_str(
                "0x0444a09d96389aa7148f1aada508e30b71299ffe650d9c97fdaae38cb9a23384",
            )?, // exchange address
            FieldElement::from_str("0xe8d4a51000")?, // percentage
            FieldElement::from_str("0x6")?, // no. of additional params
            FieldElement::from_str("0x4c97389b183b0fee4490192338bab407430078e46dfdab72b5c986bb9a2196d")?, // token 0 of pool
            FieldElement::from_str("0x78a1025dcc7077da3889ed79c0710d982b5893c007d2ff4d252e0a42a06bafa")?, // token 1 of pool
            FieldElement::from_str("0x0")?, // fee
            FieldElement::from_str("0x80")?, // tick spacing
            FieldElement::from_str(
                "0x065e8885b13c84318f43fe77280b842269b6feb6a66947f22fadc70963a14771",
            )?, // extension address
            FieldElement::from_str(&input.price_distance)?, // price distance
        ],
    };

    let result = client
        .simulate(ExecutableTransaction::Invoke(
            ExecutableInvokeTransaction::from(InvokeTransaction::V1(tx1)),
        ))
        .await
        .unwrap();
    //println!("Trace:{:#?}", result.transaction_trace);
    println!("Fee:{:#?}",result.fee_estimation);
    Ok(SimulationResult {
        token_from: input.token_from.clone(),
        token_to: input.token_to.clone(),
        from_amount: input.token_from_low.clone(),
        data_gas_consumed: result.fee_estimation.data_gas_consumed.to_string(),
        gas_consumed: result.fee_estimation.gas_consumed.to_string(),
        gas_price: result.fee_estimation.gas_price.to_string(),
        overall_fee: result.fee_estimation.overall_fee.to_string(),
        ticks_crossed: input.ticks_crossed.clone(),
    })
}

async fn simulate_transaction_v3(
    input: &SimulationInput,
    account_address: &FieldElement,
    nonce: &FieldElement
) -> Result<SimulationResult, Box<dyn std::error::Error>> {
    // Get the current nonce

    println!("Simulation for TX V3");
    let client = client();


    let tx1 = InvokeTransactionV3 {
        transaction_hash: FieldElement::from_str("0x0")?,
        nonce: *nonce,
        resource_bounds: ResourceBoundsMapping {
            l1_gas: ResourceBounds {
                max_amount: 3000,
                max_price_per_unit: 589339005056672,
            },
            l2_gas: ResourceBounds {
                max_amount: 3000,
                max_price_per_unit: 589339005056672,
            },
        },
        tip: 0,
        paymaster_data: vec![],
        account_deployment_data: vec![],
        nonce_data_availability_mode: DataAvailabilityMode::L1,
        fee_data_availability_mode: DataAvailabilityMode::L1,
        sender_address: *account_address,
        signature: Vec::new(),
        calldata: vec![
            FieldElement::from_str("0x1")?, // no. of calls
            FieldElement::from_str(
                "0x02c56e8b00dbe2a71e57472685378fc8988bba947e9a99b26a00fade2b4fe7c2",
            )?, // contract_address
            FieldElement::from_str(
                "0x01171593aa5bdadda4d6b0efde6cc94ee7649c3163d5efeb19da6c16d63a2a63",
            )?, // function selector
            FieldElement::from_str("0x17")?, // size of arguments
            FieldElement::from_str(&input.token_from)?, // token_from
            FieldElement::from_str(&input.token_from_low)?, // token from low
            FieldElement::from_str("0x0")?, // token from high
            FieldElement::from_str(&input.token_to)?, // token_to_address
            FieldElement::from_str("0x0")?, // token to low
            FieldElement::from_str("0x0")?, // token to high
            FieldElement::from_str(&input.token_to_min_low)?, // token to min low
            FieldElement::from_str("0x0")?, // token to min high
            *account_address,               // beneficiary
            FieldElement::from_str("0x0")?, // integrator fee
            *account_address,               // integrator recipient
            FieldElement::from_str("0x1")?, // no. of routes
            FieldElement::from_str(&input.token_from)?, // sell_token
            FieldElement::from_str(&input.token_to)?, // buy token
            FieldElement::from_str(
                "0x0444a09d96389aa7148f1aada508e30b71299ffe650d9c97fdaae38cb9a23384",
            )?, // exchange address
            FieldElement::from_str("0xe8d4a51000")?, // percentage
            FieldElement::from_str("0x6")?, // no. of additional params
            FieldElement::from_str("0x4c97389b183b0fee4490192338bab407430078e46dfdab72b5c986bb9a2196d")?, // token 0 of pool
            FieldElement::from_str("0x78a1025dcc7077da3889ed79c0710d982b5893c007d2ff4d252e0a42a06bafa")?, // token 1 of pool
            FieldElement::from_str("0x0")?, // fee
            FieldElement::from_str("0x80")?, // tick spacing
            FieldElement::from_str(
                "0x065e8885b13c84318f43fe77280b842269b6feb6a66947f22fadc70963a14771",
            )?, // extension address
            FieldElement::from_str(&input.price_distance)?, // price distance
        ],
    };

    let result = client
        .simulate(ExecutableTransaction::Invoke(
            ExecutableInvokeTransaction::from(InvokeTransaction::V3(tx1)),
        ))
        .await
        .unwrap();
    println!("Trace:{:#?}", result.transaction_trace);
    Ok(SimulationResult {
        token_from: input.token_from.clone(),
        token_to: input.token_to.clone(),
        from_amount: input.token_from_low.clone(),
        data_gas_consumed: result.fee_estimation.data_gas_consumed.to_string(),
        gas_consumed: result.fee_estimation.gas_consumed.to_string(),
        gas_price: result.fee_estimation.gas_price.to_string(),
        overall_fee: result.fee_estimation.overall_fee.to_string(),
        ticks_crossed: input.ticks_crossed.clone(),
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    /*
    let token0 = FieldElement::from_hex_be("0x02655c0a29305847e0d9ca1ae3045ffb8a20348d8d3a5afc71ed867a6152f227").unwrap();
    let token1 = FieldElement::from_hex_be("0x044d6289f59bf28e2d9c1924e0f6753039932b80e404d01c54ab65e0411f0548").unwrap();
    let fee = FieldElement::from_hex_be("0x0").unwrap();
    let tick_spacing = FieldElement::from_hex_be("0x80").unwrap();
    let extension = FieldElement::from_hex_be("0x65e8885b13c84318f43fe77280b842269b6feb6a66947f22fadc70963a14771").unwrap();

    
    let storage_address = storage_mapping_key("pools", &[token0, token1, fee, tick_spacing, extension]);
    println!("pools in router:{:?}", storage_address);
    let storage_address = storage_mapping_key("pools", &[token0, token1]);
    println!("pools in limit order extension:{:?}", storage_address);
    let next_tick = FieldElement::from_hex_be("0x80").unwrap();
    let storage_address = storage_mapping_key("initialized_ticks_crossed_last_crossing", &[token0, token1,next_tick]);
    println!("initialized_ticks_crossed_last_crossing in limit order extension:{:?}", storage_address);

    let owner = FieldElement::from_hex_be("0x065e8885b13c84318f43fe77280b842269b6feb6a66947f22fadc70963a14771").unwrap();
    let amount = FieldElement::from_hex_be("0x40").unwrap();
    let storage_address = storage_mapping_key("saved_balances", &[owner, token1, FieldElement::ZERO, amount]);
    println!("saved_balances in router:{:?}", storage_address);

    let lower_tick = FieldElement::from_hex_be("0x0").unwrap();
    let upper_tick = FieldElement::from_hex_be("0x80").unwrap();
    let sign = FieldElement::from_hex_be("0x0").unwrap();
    let storage_address = storage_mapping_key("ticks", &[token0, token1, fee, tick_spacing, extension, lower_tick, sign]);
    println!("saved lower in router:{:?}", storage_address);

    let storage_address = storage_mapping_key("ticks", &[token0, token1, fee, tick_spacing, extension, upper_tick, sign]);
    println!("upper tick in router:{:?}", storage_address);

    return Ok(());*/
    // Read simulation inputs from JSON file
    let file = File::open("simulation_inputs.json")?;
    let inputs: Vec<SimulationInput> = serde_json::from_reader(file)?;

    // Set account address
    let account_address = FieldElement::from_str(
        "0x059e0eaf58972c3b7de923ad6a280476430295f7ea967b768bd381bf5d90d50b",
    )?;

    // Create CSV writer for results
    let mut writer = Writer::from_path("simulation_results.csv")?;
    writer.write_record([
        "token_from",
        "token_to",
        "from_amount",
        "data_gas_consumed",
        "gas_consumed",
        "gas_price",
        "overall_fee",
        "ticks_crossed",
    ])?;

    let nonce = get_nonce(&account_address).await?;
    // Process each simulation input
    for input in inputs {
        if(input.token_from_low!="0x3f") {
            continue;
        }
        match simulate_transaction(&input, &account_address, &nonce).await {
            Ok(result) => {
                println!("Simulation Result:");
                println!("Token From: {}", result.token_from);
                println!("Token To: {}", result.token_to);
                println!("From Amount: {}", result.from_amount);
                println!("Data Gas Consumed: {}", result.data_gas_consumed);
                println!("Gas Consumed: {}", result.gas_consumed);
                println!("Gas Price: {}", result.gas_price);
                println!("Overall Fee: {}", result.overall_fee);
                println!("-------------------");

                writer.write_record([
                    &result.token_from,
                    &result.token_to,
                    &result.from_amount,
                    &result.data_gas_consumed,
                    &result.gas_consumed,
                    &result.gas_price,
                    &result.overall_fee,
                    &result.ticks_crossed,
                ])?;
                writer.flush()?;
            }
            Err(e) => {
                println!("Error simulating transaction: {:?}", e);
            }
        }
    }

    Ok(())
}
