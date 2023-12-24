use serde_json::Value;

pub fn generate_quote() -> Result<String, reqwest::Error> {
    let url = "https://api.quotable.io/quotes/random";
    let body = reqwest::blocking::get(url)?.text()?;
    let v: Value = serde_json::from_str(body.as_str()).unwrap();
    let content = v[0]["content"].as_str().unwrap();
    let author = v[0]["author"].as_str().unwrap();
    Ok(format!("{content}\n\n\t\t~{author}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn it_fetches_quote() {
        // It runs slow because it hits an API on the internet
        let ans: Result<String, reqwest::Error> =  generate_quote();
        assert!(ans.is_ok());
        println!("{}", ans.unwrap());
    }
}
