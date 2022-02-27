pub async fn get_local() -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://ipinfo.io/json").await?.text().await?;
    Ok(resp)
}

pub async fn get_ipinfo(ip: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url;
    if ip != "" {
        url = format!("https://ipinfo.io/{}/json", ip);
    } else {
        url = String::from("https://ipinfo.io/json");
    };
    let resp = reqwest::get(url).await?.text().await?;
    Ok(resp)
}
