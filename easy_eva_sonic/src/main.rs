use anyhow::{Error as E, Result};
use candle_core::{Device, Tensor};
use candle_transformers::models::quantized_llama::ModelWeights as QLlama;
use tokenizers::Tokenizer;
use std::process::Command;
use dotenvy::dotenv;
use std::time::Duration;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

mod comms;

const BASELINE_FILE: &str = "baseline.json";

// --- EISERNE WHITELIST (Diese Tools werden NIEMALS getötet) ---
const CRITICAL_WHITELIST: &[&str] = &[
    "sleep", "bash", "sh", "ps", "ls", "cargo", "rustc", "apt", "pkg", "tar", "gzip"
];

fn get_processes_with_pid() -> Vec<(String, String)> {
    let mut results = Vec::new();
    if let Ok(entries) = fs::read_dir("/proc") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let pid = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
                if pid.chars().all(|c| c.is_numeric()) {
                    if let Ok(cmdline) = fs::read_to_string(path.join("cmdline")) {
                        let full_cmd = cmdline.replace('\0', " ").trim().to_string();
                        if !full_cmd.is_empty() && !full_cmd.contains("easy_eva") {
                            results.push((full_cmd, pid.to_string()));
                        }
                    }
                }
            }
        }
    }
    results
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

    pub fn analyze_with_confidence(&mut self, cmd: &str, is_in_baseline: bool) -> Result<(String, f32)> {
        let prompt = format!("<|system|>\nSecurity expert. Respond ONLY with SAFE or RISK.\n<|user|>\nIs this dangerous: {}\n<|assistant|>\nVerdict:", cmd);
        let tokens = self.tokenizer.encode(prompt, true).map_err(E::msg)?;
        let input = Tensor::new(tokens.get_ids(), &self.device)?.unsqueeze(0)?;
        let mut output_tokens = Vec::new();
        let mut current_input = input.clone();
        for _ in 0..6 {
            let logits = self.model.forward(&current_input, output_tokens.len())?;
            let logits = logits.squeeze(0)?;
            let next_token = if logits.rank() >= 2 {
                let (seq_len, _) = logits.dims2()?;
                logits.get(seq_len - 1)?.argmax(0)?.to_scalar::<u32>()?
            } else { logits.argmax(0)?.to_scalar::<u32>()? };
            if next_token == 2 { break; } 
            output_tokens.push(next_token);
            current_input = Tensor::new(&[next_token], &self.device)?.unsqueeze(0)?;
        }
        let verdict = self.tokenizer.decode(&output_tokens, true).map_err(E::msg)?.to_uppercase();

        let mut confidence: f32 = 0.4; // Basis gesenkt
        if !is_in_baseline { confidence += 0.3; }
        if cmd.contains("/home") || cmd.starts_with("./") { confidence += 0.2; }
        if verdict.contains("RISK") { confidence += 0.1; }

        Ok((verdict, confidence.clamp(0.0, 1.0)))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    println!("🛡️ EASY-EVA SONIC v10.8.11 [SAFETY-FIRST]");
    
    let mut ai = LocalAIModule::load("../assets/tinyllama.gguf")?;
    let mut baseline: HashSet<String> = HashSet::new();

    if Path::new(BASELINE_FILE).exists() {
        let data = fs::read_to_string(BASELINE_FILE)?;
        baseline = serde_json::from_str(&data).unwrap_or_default();
        println!("💾 Baseline geladen.");
    } else {
        println!("🎓 Lernen...");
        for _ in 1..=5 {
            for (cmd, _) in get_processes_with_pid() {
                let exe = cmd.split_whitespace().next().unwrap_or("").split('/').last().unwrap_or("").to_string();
                baseline.insert(exe);
            }
            tokio::time::sleep(Duration::from_secs(3)).await;
        }
        fs::write(BASELINE_FILE, serde_json::to_string(&baseline)?)?;
        println!("✅ Baseline gespeichert.");
    }

    loop {
        let procs = get_processes_with_pid();
        for (cmd, pid) in procs {
            let exe = cmd.split_whitespace().next().unwrap_or("").split('/').last().unwrap_or("").to_string();
            
            // 1. Check Whitelist
            if CRITICAL_WHITELIST.contains(&exe.as_str()) {
                continue; 
            }

            if !baseline.contains(&exe) || cmd.contains("nmap") {
                if exe.contains("cargo") || exe.contains("bash") { continue; }

                let (verdict, confidence) = ai.analyze_with_confidence(&cmd, baseline.contains(&exe))?;
                let trust = (confidence * 100.0) as u32;

                // 2. Kill nur bei sehr hoher Konfidenz (>95%)
                if trust >= 95 {
                    println!("[{}%] 💀 TERMINATE: {} (PID: {})", trust, cmd, pid);
                    let _ = Command::new("kill").arg("-9").arg(&pid).output();
                } else if trust >= 70 {
                    println!("[{}%] ⚠️  SUSPICIOUS: {}", trust, cmd);
                }
            }
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

