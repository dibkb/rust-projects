use std::{error::Error};
use reqwest::{blocking::Client, dns::Resolve};
use scraper::Html;
// use serde::{Serialize, Deserialize};

// #[derive(Serialize, Deserialize)]


struct AmazonScraper {
    client: Client,
    base_url: String,
    document: Option<Html>,
    current_asin: String
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

    fn get_page_html(&mut self, asin: &str) -> Result<(), Box<dyn Error>> {
        let url = format!("{}{}", self.base_url, asin);
        
        if let Some(_) = &self.document {
            if self.current_asin == asin {
                println!("Using cached HTML for ASIN: {}", asin);
                return Ok(());
            }
        }
        
        println!("Fetching new HTML for ASIN: {}", asin);
        let response = self.client.get(&url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
            .send()?;
        
        let content = response.text()?;
        self.document = Some(scraper::Html::parse_document(&content));
        self.current_asin = asin.to_string();
        Ok(())
    }

    fn get_product_title(&mut self, asin: &str) -> Result<String, Box<dyn Error>> {
        self.get_page_html(asin)?;

        let document = self.document.as_ref()
            .ok_or_else(|| "Document not loaded. Call get_page_html first.")?;
        
        let title_selector = scraper::Selector::parse("span#productTitle").unwrap();
        let alt_title_selector = scraper::Selector::parse("span#title.a-size-small").unwrap();

        let title = if let Some(title_element) = document.select(&title_selector).next() {
            title_element.text().collect::<Vec<_>>().join("").trim().to_string()
        } else if let Some(alt_title_element) = document.select(&alt_title_selector).next() {
            alt_title_element.text().collect::<Vec<_>>().join("").trim().to_string()
        } else {
            "Title not found".to_string()
        };

        Ok(title)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut scraper = AmazonScraper::new("B0DMWFVQDX".to_string());
    let title = scraper.get_product_title("B0DMWFVQDX")?;
    println!("Product Title: {}", title);
    Ok(())
}