use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
struct Quote {
    content: String,
    author: String,
}

pub fn generate_quote() -> Result<String> {
    let url = "https://api.quotable.io/quotes/random";
    let payload = reqwest::blocking::get(url)?.json::<Vec<Quote>>()?;
    let q = payload.get(0).expect("No quotes given by API.");
    Ok(format!("\"{}\"\n\n\t\t~{}", q.content, q.author))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn it_fetches_quote_json() {
        let ans = generate_quote();
        match ans {
            Err(e) => println!("Got some error! {e}"),
            Ok(s) => println!("Got a result: \n{s}"),
        }
    }
}
