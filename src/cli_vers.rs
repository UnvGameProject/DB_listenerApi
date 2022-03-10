//this is an updated cli app the project defines news api and moves some modules
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
use std::error::Error;
use dotenv::dotenv;

use dbevent_api::{NewsAPI, Endpoint, Country, Article, NewsAPIResponse};
use crate::theme;

fn render_articles(articles: &Vec<Article>){ //iterate through response and print to cli
    let theme = theme::default();
    theme.print_text("# Top headlines\n\n");
    for i in articles { 
        theme.print_text(&format!("'{}'", i.title()));
        theme.print_text(&format!("> *{}*", i.url()));
        theme.print_text("---");
    }
}



#[tokio::main]
pub async fn run() -> Result<(), Box<dyn Error>> {
    dotenv()?;

    let api_key = std::env::var("API_KEY")?;
    let mut newsapi = NewsAPI::new(&api_key);
    newsapi.endpoint(Endpoint::TopHeadlines).country(Country::Us);
    let newsapi_response: NewsAPIResponse = newsapi.fetch_async().await?;

    //let mut db_post = newsapi_response;      //make db_insert a parameter of Vec! 
    //
    //let db_postupdate: NewsAPIResponse = dbpost.db_insert(&newsapi_response.articles()).await?;
    render_articles(&newsapi_response.articles());

    Ok(())
}