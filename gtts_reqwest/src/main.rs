#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .build()?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Cookie", "1P_JAR=2024-04-18-06; NID=513=hHbodNqbdmZlxoSTWCQ5AxlEgqVhrUzpRQZXMWV1t2Pj-zFGMG_-EKofBA99k_0lCiaoFomKiO6J-_Kdo83rt9Uc-dDqvxQLxjWGu5AT07hnsJVSVy6v9H7FGBWUXjziT3dMs-9t9LeodUeTXUkgTSGhJPKPCQrU2QLoyZTUpa0".parse()?);

    let request = client.request(reqwest::Method::GET, "https://translate.google.com/translate_tts?ie=UTF-8&q=hello&tl=en&total=1&idx=0&textlen=5&tl=en&client=tw-ob")
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    // println!("{}", body);

    Ok(())
}