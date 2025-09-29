use ollama_rs::Ollama;
use ollama_rs::generation::completion::request::GenerationRequest;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ollama = Ollama::default();
    let model = "llama3:latest".to_string();
    let prompt = "中华人民共和国的首都是什么?".to_string();

    let res = ollama.generate(GenerationRequest::new(model, prompt)).await;

    if let Ok(res) = res {
        println!("{}", res.response);
    }

    Ok(())
}
