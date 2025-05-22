use std::{error::Error};
use reqwest::{blocking::Client, dns::Resolve};
use scraper::Html;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct PriceData {
    desktop_buybox_group_1: Vec<BuyBoxItem>
}

#[derive(Debug, Serialize, Deserialize)]
struct BuyBoxItem {
    displayPrice: String,
    priceAmount: f64,
    currencySymbol: String,
    integerValue: String,
    decimalSeparator: String,
    fractionalValue: String,
    symbolPosition: String,
    hasSpace: bool,
    showFractionalPartIfEmpty: bool,
    offerListingId: String,
    locale: String,
    buyingOptionType: String,
    aapiBuyingOptionIndex: i32
}

struct AmazonScraper {
    client: Client,
    base_url: String,
    document: Option<Html>,
    current_asin: String
}
#[derive(Debug)]
struct Price {
    amount : f64,
    currency : String
}
impl AmazonScraper {
    fn new(asin: String) -> Self {
        let client: Client = Client::new();
        Self {
            client,
            base_url: "https://www.amazon.in/dp/".to_string(),
            document: None,
            current_asin: asin
        }
    }

    fn get_page_html(&mut self) -> Result<(), Box<dyn Error>> {
        let url = format!("{}{}", self.base_url, self.current_asin);
        
        if let Some(_) = &self.document {
            println!("Using cached HTML for ASIN: {}", self.current_asin);
            return Ok(());
            
        }
        
        println!("Fetching new HTML for ASIN: {}", self.current_asin);
        let response = self.client.get(&url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
            .send()?;
        
        let content = response.text()?;
        self.document = Some(scraper::Html::parse_document(&content));
        self.current_asin = self.current_asin.to_string();
        Ok(())
    }

    fn get_product_title(&mut self) -> Result<String, Box<dyn Error>> {
        self.get_page_html()?;

        let document = self.document.as_ref()
            .ok_or_else(|| "Document not loaded. Call get_page_html first.")?;
        
        let title_selector = scraper::Selector::parse("span#productTitle").unwrap();
        let alt_title_selector = scraper::Selector::parse("span#title.a-size-small").unwrap();

        let title: String = if let Some(title_element) = document.select(&title_selector).next() {
            title_element.text().collect::<Vec<_>>().join("").trim().to_string()
        } else if let Some(alt_title_element) = document.select(&alt_title_selector).next() {
            alt_title_element.text().collect::<Vec<_>>().join("").trim().to_string()
        } else {
            "Title not found".to_string()
        };

        Ok(title)
    }

    fn get_price(&self) -> Result<Price, Box<dyn Error>> {
        let document = self.document.as_ref()
            .ok_or_else(|| "Document not loaded. Call get_page_html first.")?;
        
        let price_selector = scraper::Selector::parse("div.a-section.aok-hidden.twister-plus-buying-options-price-data").unwrap();
        
        if let Some(price_element) = document.select(&price_selector).next() {
            let json_str = price_element.text().collect::<Vec<_>>().join(" ");
            

            let price_data: PriceData = serde_json::from_str(&json_str)?;
            
            if let Some(item) = price_data.desktop_buybox_group_1.first() {
                let result = Price {
                    amount: item.priceAmount,
                    currency: item.currencySymbol.to_string(),
                };
                Ok(result)
            } else {
                Err("No price data found".into())
            }
        } else {
            Err("Price element not found".into())
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut scraper = AmazonScraper::new("B0DMWFVQDX".to_string());
    let title = scraper.get_product_title()?;
    println!("Product Title: {}", title);
    let res = scraper.get_price()?;
    println!("{:?}",res);
    Ok(())
}