use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut paths:Vec<String> = vec!["/".to_owned()];
    let mut visited:Vec<String> = vec![];
    let base_url = "https://nwapw.org/Java";

    println!("All paths accessible from the table of contents: ");

    while paths.len() != 0 {
        let path = paths.pop().unwrap();
        if visited.contains(&path) {
            continue;
        }
        println!("{}", path);
        

        let html = reqwest::get(format!("{}{}", base_url, path))
        .await?
        .text()
        .await?;
        
        let fragment = Html::parse_fragment(&html);
        let selector = Selector::parse("a").unwrap();

        for element in fragment.select(&selector) { 
            if let Some(s) = element.value().attr("href") {
                let mut s = s.to_owned();
                //Strip # fragments from everything
                if let Some(i) =  s.find("#") {
                    s = s.chars().take(i).collect();
                }
                //Remove outward links
                if let Some(_) = s.find("mailto") {
                    continue;
                }
                if let Some(_) = s.find("http") {
                    continue;
                }
                if !s.is_empty() {
                    paths.push(s.to_owned())
                }
            }
        }

        visited.push(path);
    }
    Ok(())
}
