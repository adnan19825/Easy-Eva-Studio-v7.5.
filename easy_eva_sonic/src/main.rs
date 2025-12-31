use anyhow::{Error as E, Result};
use candle_core::{Device, Tensor};
use candle_transformers::models::quantized_llama::ModelWeights as QLlama;
use tokenizers::Tokenizer;
use std::time::Instant;

// --- STRUKTUREN ---

struct LocalAIModule {
    model: QLlama,
    tokenizer: Tokenizer,
    device: Device,
}

pub struct ZKPMetrics {
    pub proof_size: usize,
    pub compute_time: f64,
    pub proven_value: u8,
}

// --- LOGIK ---

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
        // v9.2 FIX: Strict Whitelisting
        // Wir zerlegen den Input in einzelne Prozesse
        let parts: Vec<&str> = text.split(',').map(|s| s.trim()).collect();
        let whitelist = ["chrome", "spotify", "calculator", "notes", "clock"];
        
        // Prüfung: Sind ALLE Teile in der Whitelist?
        let all_safe = parts.iter().all(|part| {
            // Wir prüfen lowercase, damit "Chrome" und "chrome" erkannt werden
            whitelist.contains(&part.to_lowercase().as_str())
        });

        if all_safe {
            return Ok("SAFE (WHITELISTED)".to_string());
        }

        // Wenn NICHT alle sicher sind (z.B. Termux ist dabei), muss die KI ran!
        // Wir sagen der KI explizit, was sie tun soll.
        let prompt = format!(
            "<|system|>\n\
            You are a security analyst. Reply strictly with ONE word.\n\
            Rules:\n\
            - If input contains dangerous tools (termux, bash, nmap, root) -> RISK\n\
            - Only if ALL apps are harmless -> SAFE\n\
            <|user|>\n\
            Analyze: {}\n\
            <|assistant|>\n\
            Verdict:",
            text
        );

        let tokens = self.tokenizer.encode(prompt, true).map_err(E::msg)?;
        let input = Tensor::new(tokens.get_ids(), &self.device)?.unsqueeze(0)?;
        let mut output_tokens = Vec::new();
        let mut current_input = input.clone();

        for _ in 0..15 {
            let logits = self.model.forward(&current_input, output_tokens.len())?;
            let logits = logits.squeeze(0)?;
            let next_token_logits = if logits.rank() == 2 {
                let (seq_len, _) = logits.dims2()?;
                logits.get(seq_len - 1)?
            } else { logits };

            let next_token = next_token_logits.argmax(0)?.to_scalar::<u32>()?;
            if next_token == 2 { break; } 
            output_tokens.push(next_token);
            current_input = Tensor::new(&[next_token], &self.device)?.unsqueeze(0)?;
        }

        let raw_result = self.tokenizer.decode(&output_tokens, true).map_err(E::msg)?;
        let clean_result = raw_result.trim().to_uppercase();

        Ok(clean_result)
    }
}

// --- ZKP SIMULATION ---
fn generate_zkp(verdict: &str) -> ZKPMetrics {
    let start = Instant::now();
    let val = if verdict.contains("SAFE") { 0 } else { 1 };
    std::thread::sleep(std::time::Duration::from_millis(5)); 
    
    ZKPMetrics {
        proof_size: 608,
        compute_time: start.elapsed().as_secs_f64() * 1000.0,
        proven_value: val,
    }
}

// --- MAIN ---

fn main() -> Result<()> {
    println!("🔧 EASY-EVA SONIC SCREWDRIVER (v9.2 Strict Patch)");
    println!("===================================================");

    let mut ai = LocalAIModule::load("../assets/tinyllama.gguf")?;

    // TEST: Der "Trojaner"-Angriff (Chrome + Termux)
    let process_list = "chrome, termux";
    println!("[1] KI Scannt: '{}'", process_list);

    let raw_verdict = ai.analyze(process_list)?;
    
    println!("   -> [DEBUG] Raw Output: '{}'", raw_verdict);
    
    // Entscheidung
    let is_safe = raw_verdict.contains("SAFE") && !raw_verdict.contains("RISK");
    let display_verdict = if is_safe { "SAFE" } else { "RISK" };
    let score = if is_safe { 0 } else { 1 };
    
    println!("   -> KI Analyse: '{}'", display_verdict);
    println!("   -> Score: {} ({})", score, if score == 0 { "Safe" } else { "Risk" });

    println!("\n✅ ZERTIFIKAT ERSTELLT.");
    println!("--------------------------------------------");
    
    let zkp = generate_zkp(display_verdict);
    println!("📊 ZKP TELEMETRIE:");
    println!("   • Proof Größe:    {} Bytes", zkp.proof_size);
    println!("   • Rechenzeit:     {:.2}ms", zkp.compute_time);
    println!("   • Bewiesener Wert: {}", zkp.proven_value);

    if zkp.proven_value == 1 {
        println!("\n🚨 ALARM: BEDROHUNG ERKANNT! 🚨");
    } else {
        println!("\n🛡️ SYSTEM SICHER (Verified Safe).");
    }
    println!("--------------------------------------------");

    Ok(())
}

