use std::process;
pub use super::super::config::get_config;
use scraper::{Html, Selector};

pub fn pages(base: &str, pages: u32) -> Vec<String> {
    let mut pages_links: Vec<String> = vec![];
    for page in 1..=pages {
        pages_links.push(format!("{base}/{page}"));
    }

    pages_links
}

pub async fn scrape_page(url: &str) -> String {
    let resp = reqwest::get(url).await.unwrap();
    if resp.status() != 200 {
        println!("{}", resp.status());
        process::exit(0);
    }
    let html = resp.text().await.unwrap();
    let doc = Html::parse_fragment(&html);
    let selector = Selector::parse(".m-b-sm").unwrap();
    for el in doc.select(&selector) {
        println!("title:{} ", el.inner_html().split('-').next().unwrap());
    }
    html
}
