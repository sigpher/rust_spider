use scraper::{Html, Selector};
use serde::Deserialize;
use serde::Serialize;
use std::{error::Error, fs, process};
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config();
    // let base = config.get_string("base").unwrap();
    // let total_pages = config.get_int("pages").unwrap() as u32;
    let base = config.base;
    let total_pages = config.pages;

    // let links = pages("https://ssr1.scrape.center/page", 10);
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

fn pages(base: &str, pages: u32) -> Vec<String> {
    let mut pages_links: Vec<String> = vec![];
    for page in 1..=pages {
        pages_links.push(format!("{base}/{page}"));
    }

    pages_links
}

async fn scrape_page(url: &str) -> String {
    let resp = reqwest::get(url).await.unwrap();
    if resp.status() != 200 {
        println!("{}", resp.status());
        process::exit(0);
    }
    let html = resp.text().await.unwrap();
    let doc = Html::parse_fragment(&html);
    let selector = Selector::parse(".m-b-sm").unwrap();
    for el in doc.select(&selector) {
        println!("title:{}", el.inner_html());
    }
    html
}

fn get_config() -> SettingsStruct {
    // let settings = Config::builder()
    //     .add_source(config::File::with_name("settings"))
    //     .build()
    //     .unwrap();
    let settings = toml::from_str(&fs::read_to_string("settings.toml").unwrap()).unwrap();
    settings
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SettingsStruct {
    pub base: String,
    pub pages: u32,
}
