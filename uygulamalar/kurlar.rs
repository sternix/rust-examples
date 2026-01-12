/*
[dependencies]
reqwest = { version = "0.11"}
quick-xml = { version = "0.31", features = [ "serialize" ] }
serde = {version = "1.0", features = ["derive"]}
tokio = {version = "1", features=["full"]}
*/

use quick_xml::de::from_str;
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct Kurlar {
    #[serde(rename = "@Tarih")]
    Tarih: String,
    #[serde(rename = "@Date")]
    Date: String,
    #[serde(rename = "@Bulten_No")]
    BultenNo: String,
    #[serde(rename = "Currency")]
    Currencies: Vec<Currency>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct Currency {
    #[serde(rename = "@CrossOrder")]
    CrossOrder: String,
    #[serde(rename = "@Kod")]
    Kod: String,
    #[serde(rename = "@CurrencyCode")]
    CurrencyCode: String,
    Unit: String,
    Isim: String,
    CurrencyName: String,
    ForexBuying: String,
    ForexSelling: String,
    BanknoteBuying: String,
    BanknoteSelling: String,
    CrossRateUSD: String,
    CrossRateOther: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new();

    let xml = client
        .get("https://www.tcmb.gov.tr/kurlar/today.xml")
        .send()
        .await?
        .text()
        .await?;

    let kurlar: Kurlar = from_str(&xml).unwrap();

    println!(
        "Tarih: {}, Date: {} Bülten No: {}",
        kurlar.Tarih, kurlar.Date, kurlar.BultenNo
    );

    for kur in kurlar.Currencies {
        println!(
            "CrossOrder: {}, Kod: {}, CurrencyCode: {}",
            kur.CrossOrder, kur.Kod, kur.CurrencyCode,
        );
        println!("Unit: {}", kur.Unit);
        println!("Isim: {}", kur.Isim);
        println!("CurrencyName: {}", kur.CurrencyName);
        println!("ForexBuying: {}", kur.ForexBuying);
        println!("ForexSelling: {}", kur.ForexSelling);
        println!("BanknoteBuying: {}", kur.BanknoteBuying);
        println!("BanknoteSelling: {}", kur.BanknoteSelling);
        println!("CrossRateUSD: {}", kur.CrossRateUSD);
        println!("CrossRateOther: {}", kur.CrossRateOther);

        println!("--------8<-------------------");
    }

    Ok(())
}
