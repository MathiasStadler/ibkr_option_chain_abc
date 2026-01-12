use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct StockContract {
    conid: i64,
    symbol: String,
}

#[derive(Debug)]
struct OptionChain {
    expirations: Vec<String>,
    strikes: Vec<f64>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starte IBKR Option Chain Download für IBM");

    let client = Client::builder().cookie_store(true).build()?;

    let stock = get_stock_conid(&client, "IBM").await?;
    println!("IBM conid: {}", stock.conid);

    let option_chain = get_option_chain(&client, stock.conid).await?;

    println!("\nExpirations:");
    for e in &option_chain.expirations {
        println!("  {}", e);
    }

    println!("\nStrikes:");
    for s in &option_chain.strikes {
        println!("  {}", s);
    }

    Ok(())
}

async fn get_stock_conid(
    client: &Client,
    symbol: &str,
) -> Result<StockContract, Box<dyn std::error::Error>> {
    let url = format!(
        "http://localhost:5000/v1/api/iserver/secdef/search?symbol={}&secType=STK",
        symbol
    );

    let resp: Vec<StockContract> = client.get(url).send().await?.json().await?;

    resp.into_iter()
        .next()
        .ok_or_else(|| "Kein Aktien-Contract gefunden".into())
}

async fn get_option_chain(
    client: &Client,
    conid: i64,
) -> Result<OptionChain, Box<dyn std::error::Error>> {
    let url = format!(
        "http://localhost:5000/v1/api/iserver/secdef/info?conid={}&sectype=OPT",
        conid
    );

    let resp: Vec<serde_json::Value> = client.get(url).send().await?.json().await?;

    let mut expirations = Vec::new();
    let mut strikes = Vec::new();

    for item in resp {
        if let Some(exp) = item.get("maturityDate").and_then(|v| v.as_str()) {
            expirations.push(exp.to_string());
        }
        if let Some(strike) = item.get("strike").and_then(|v| v.as_f64()) {
            strikes.push(strike);
        }
    }

    expirations.sort();
    expirations.dedup();
    strikes.sort_by(|a, b| a.partial_cmp(b).unwrap());
    strikes.dedup();

    Ok(OptionChain {
        expirations,
        strikes,
    })
}
