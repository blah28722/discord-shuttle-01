use anyhow::Result;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct Quote {
    _id: String,
    content: String,
    author: String,
    tags: Vec<String>,
    authorSlug: String,
    length: u32,
    dateAdded: String,
    dateModified: String,
}

#[allow(dead_code)]
pub fn generate_quote() -> Result<String> {
    let url = "https://api.quotable.io/quotes/random";
    let data = reqwest::blocking::get(url)?.text()?;
    let payload = serde_json::from_str::<Vec<Quote>>(data.as_str())?;
    match payload.get(0) {
        None => Ok(format!("<Error: missing/bad response from API server>")),
        Some(quote) => {
            let content = &quote.content;
            let author = &quote.author;
            Ok(format!("\"{content}\"\n\n\t\t~{author}"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn it_fetches_quote_json() {
        let ans = generate_quote();
        match ans {
            Err(e) => {
                println!("Got some error! {e}");
            }
            Ok(s) => {
                println!("Got a result: \n{s}");
            }
        }
    }
}
