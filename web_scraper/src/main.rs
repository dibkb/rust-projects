use std::error::Error;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Movie {
    title: String,
    year: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let response = reqwest::blocking::get(
        "https://www.imdb.com/search/title/?groups=top_100&sort=user_rating,desc&count=10",
    )?;
    println!("{}", response.status());
    let body = response.text()?;

    let document = scraper::Html::parse_document(&body);

    let selector: scraper::Selector = scraper::Selector::parse(".ipc-metadata-list-summary-item").unwrap();
    let items: Vec<_> = document.select(&selector).collect();

    let mut movies = Vec::new();

    for item in items {
        let item_vec = item.text().collect::<Vec<_>>();
        if item_vec.len() == 13 {
            if let (Some(title), Some(year)) = (item_vec.get(0), item_vec.get(1)) {
                movies.push(Movie {
                    title: title.trim().to_string(),
                    year: year.trim().to_string(),
                });
            }
        }
    }

    let json = serde_json::to_string_pretty(&movies)?;
    println!("{}", json);

    Ok(())
}
