use std::error::Error;

use rust_spider::{config::get_config, scrape::scrape::{pages, scrape_page}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config()?;
    let base = config.base;
    let total_pages = config.pages;

    let links = pages(&base, total_pages);
    let mut fs = Vec::new();
    for link in links {
        let h = tokio::spawn(async move {
            scrape_page(&link).await;
        });
        fs.push(h);
    }
    for f in fs {
        let _handle = tokio::join!(f);
    }
    Ok(())
}

