use std::error::Error;

use serde::Deserialize;
use colour::{dark_green,yellow};

// followed https://www.youtube.com/watch?v=4km2UijVC3M

#[derive(Deserialize)]
struct Articles {
    articles: Vec<Article>
}

#[derive(Deserialize)]
struct Article {
    title: String,
    url: String
}

fn get_articles(url: &str) -> Result<Articles,Box<dyn Error>> {

    let response = ureq::get(url).call()?.into_string()?;

    let articles: Articles = serde_json::from_str(&response)?;

    Ok(articles)
}

fn  render_articles(articles: &Articles) {
    for article in &articles.articles {
        dark_green!("> {}\n", article.title);
        yellow!("- {}\n\n", article.url);
    }
    
}

fn main() -> Result<(), Box<dyn Error>> {
    let api_key: &str = "ENTER_YOUR_KEY";
    let base_url: &str = "https://newsapi.org/v2/top-headlines?country=us&apiKey=";
    let url: &String = &format!("{}{}", base_url, api_key);
    let articles: Articles = get_articles(url)?;
    
    render_articles(&articles);

    Ok(())
}
