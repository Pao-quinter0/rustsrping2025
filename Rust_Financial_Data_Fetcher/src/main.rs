use serde::Deserialize;
use std::fs::OpenOptions;
use std::io::Write;
use std::{thread, time};

trait Pricing {
    fn fetch_price(&mut self) -> Result<(), String>;
    fn save_to_file(&self);
}

#[derive(Deserialize, Debug)]
struct CombinedData {
    bitcoin: CoinPrice,
    ethereum: CoinPrice,
}

#[derive(Deserialize, Debug)]
struct CoinPrice {
    usd: f64,
}
// Bitcoin
struct Bitcoin {
    price: f64,
}

impl Pricing for Bitcoin {
    fn fetch_price(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn save_to_file(&self) {
        save_price_to_file("bitcoin.txt", self.price);
    }
}
// Ethereum
struct Ethereum {
    price: f64,
}

impl Pricing for Ethereum {
    fn fetch_price(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn save_to_file(&self) {
        save_price_to_file("ethereum.txt", self.price);
    }
}
// SP500
#[derive(Deserialize, Debug)]
struct SP500ChartData {
    chart: Chart,
}

#[derive(Deserialize, Debug)]
struct Chart {
    result: Vec<ChartResult>,
}

#[derive(Deserialize, Debug)]
struct ChartResult {
    indicators: Indicators,
}

#[derive(Deserialize, Debug)]
struct Indicators {
    quote: Vec<Quote>,
}

#[derive(Deserialize, Debug)]
struct Quote {
    close: Vec<Option<f64>>,
}

struct SP500 {
    price: f64,
}

impl Pricing for SP500 {
    fn fetch_price(&mut self) -> Result<(), String> {
        let url = "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m";
        let response = ureq::get(url)
            .set("User-Agent", "Mozilla/5.0")
            .call()
            .map_err(|e| format!("S&P 500 fetch error: {}", e))?;

        let reader = response.into_reader();

        let data: SP500ChartData =
            serde_json::from_reader(reader).map_err(|e| format!("Parse error: {}", e))?;

        if let Some(latest_price) = data
            .chart
            .result
            .first()
            .and_then(|r| r.indicators.quote.first())
            .and_then(|q| q.close.iter().rev().flatten().next())
        {
            self.price = *latest_price;
            Ok(())
        } else {
            Err("No valid price found in SP500 data.".to_string())
        }
    }

    fn save_to_file(&self) {
        save_price_to_file("sp500.txt", self.price);
    }
}
fn save_price_to_file(filename: &str, price: f64) {
    let timestamp = chrono::Utc::now().to_rfc3339();
    let line = format!("{},{}\n", timestamp, price);

    if let Ok(mut file) = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)
    {
        let _ = file.write_all(line.as_bytes());
    } else {
        eprintln!("Failed to write to {}", filename);
    }
}
fn main() {
    let mut btc = Bitcoin { price: 0.0 };
    let mut eth = Ethereum { price: 0.0 };
    let mut sp500 = SP500 { price: 0.0 };

    loop {
        println!("Fetching data...");

        let gecko_url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin,ethereum&vs_currencies=usd";
        match ureq::get(gecko_url).call() {
            Ok(response) => {
                let reader = response.into_reader();
                match serde_json::from_reader::<_, CombinedData>(reader) {
                    Ok(data) => {
                        btc.price = data.bitcoin.usd;
                        eth.price = data.ethereum.usd;
                        btc.save_to_file();
                        eth.save_to_file();
                    }
                    Err(e) => eprintln!("Parse error (CoinGecko): {}", e),
                }
            }
            Err(e) => eprintln!("Fetch error (CoinGecko): {}", e),
        }

        thread::sleep(time::Duration::from_secs(2));

        if let Err(e) = sp500.fetch_price() {
            eprintln!("{}", e);
        } else {
            sp500.save_to_file();
        }

        println!("Data saved. Sleeping 10 seconds...\n");
        thread::sleep(time::Duration::from_secs(10));
    }
}
