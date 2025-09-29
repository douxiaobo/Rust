use ollama_rs::Ollama;
use ollama_rs::generation::completion::request::GenerationRequest;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("正在初始化 Ollama 客户端...");
    let ollama = Ollama::default();
    let model = "llama3:latest".to_string();
    let prompt = "中华人民共和国的首都是什么?".to_string();

    println!("正在向模型发送请求: {}", model);
    println!("提示词: {}", prompt);

    match ollama.generate(GenerationRequest::new(model, prompt)).await {
        Ok(response) => {
            println!("收到响应:");
            println!("{}", response.response);
        }
        Err(e) => {
            eprintln!("请求失败: {:?}", e);
        }
    }

    Ok(())
}
