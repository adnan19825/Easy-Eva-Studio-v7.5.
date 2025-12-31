use anyhow::{Error as E, Result};
use candle_core::{Device, Tensor};
use candle_transformers::models::quantized_llama::ModelWeights as QLlama;
use tokenizers::Tokenizer;
use std::process::Command;
use dotenvy::dotenv;

mod comms;

// --- Identitäts-Logik ---
fn get_node_info() -> String {
    let hostname = Command::new("hostname").output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());
        
    // Wir holen die lokale IP (für Android/Termux optimiert)
    let ip = Command::new("sh").arg("-c").arg("ifconfig | grep 'inet ' | grep -v '127.0.0.1' | awk '{print $2}' | head -n 1")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "no-ip".to_string());

    format!("Node: {} | IP: {}", hostname, ip)
}

struct LocalAIModule {
    model: QLlama,
    tokenizer: Tokenizer,
    device: Device,
}

impl LocalAIModule {
    pub fn load(model_path: &str) -> Result<Self> {
        let device = Device::Cpu;
        let mut file = std::fs::File::open(model_path)?;
        let content = candle_core::quantized::gguf_file::Content::read(&mut file)?;
        let model = QLlama::from_gguf(content, &mut file, &device)?;
        let tokenizer = Tokenizer::from_file("../assets/tokenizer.json").map_err(E::msg)?;
        Ok(Self { model, tokenizer, device })
    }

    pub fn analyze(&mut self, text: &str) -> Result<String> {
        let prompt = format!("<|system|>\nSecurity analyst. ONE word.\n<|user|>\nAnalyze: {}\n<|assistant|>\nVerdict:", text);
        let tokens = self.tokenizer.encode(prompt, true).map_err(E::msg)?;
        let input = Tensor::new(tokens.get_ids(), &self.device)?.unsqueeze(0)?;
        let mut output_tokens = Vec::new();
        let mut current_input = input.clone();

        for _ in 0..10 {
            let logits = self.model.forward(&current_input, output_tokens.len())?;
            let next_token = logits.squeeze(0)?.argmax(logits.rank()-1)?.to_scalar::<u32>()?;
            if next_token == 2 { break; } 
            output_tokens.push(next_token);
            current_input = Tensor::new(&[next_token], &self.device)?.unsqueeze(0)?;
        }
        Ok(self.tokenizer.decode(&output_tokens, true).map_err(E::msg)?.trim().to_uppercase())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok(); // Lädt GITHUB_TOKEN aus .env
    
    let info = get_node_info();
    println!("🔧 EASY-EVA SONIC (v10.2 Identity & IP)");
    println!("📍 {}", info);

    let mut ai = LocalAIModule::load("../assets/tinyllama.gguf")?;
    let process_list = "chrome, termux, nmap";
    
    let raw_verdict = ai.analyze(process_list)?;
    let is_safe = raw_verdict.contains("SAFE") && !raw_verdict.contains("RISK");
    let verdict = if is_safe { "SAFE" } else { "RISK" };

    if verdict == "RISK" {
        println!("🚨 ALARM!");
        if let Ok(token) = std::env::var("GITHUB_TOKEN") {
            let reporter = comms::GitHubReporter::new(token)?;
            let details = format!("System: {}\nProzesse: {}", info, process_list);
            let url = reporter.create_security_alert(verdict, 1, &details).await?;
            println!("✅ Alert gesendet: {}", url);
        }
    } else {
        println!("🛡️ System sicher.");
    }

    Ok(())
}

