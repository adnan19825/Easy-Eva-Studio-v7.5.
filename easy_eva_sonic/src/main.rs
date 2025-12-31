use anyhow::{Error as E, Result};
use candle_transformers::models::quantized_llama::ModelWeights as QGemma;
use candle_core::{Device, Tensor};
use candle_core::quantized::gguf_file;
use tokenizers::Tokenizer;
use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};
use curve25519_dalek_ng::scalar::Scalar;
use merlin::Transcript;
use rand::{thread_rng, RngCore};
// Neue Imports für Metriken
use sha2::{Sha256, Digest};
use std::time::Instant;

// --- TEIL 1: KI MODUL ---
pub struct LocalAIModule {
    model: QGemma,
    tokenizer: Tokenizer,
    device: Device,
}

impl LocalAIModule {
    pub fn load(model_path: &str) -> Result<Self> {
        println!("🚀 [AI-Core] Lade TinyLlama (v8.0 Platinum)...");
        let device = Device::Cpu;
        let mut file = std::fs::File::open(model_path)?;
        
        let content = gguf_file::Content::read(&mut file)?;
        let model = QGemma::from_gguf(content, &mut file, &device)?;
        
        let tokenizer = Tokenizer::from_file("../assets/tokenizer.json").map_err(E::msg)?;
        Ok(Self { model, tokenizer, device })
    }

    pub fn analyze(&mut self, text: &str) -> Result<String> {
        let prompt = format!(
            "<|system|>\nAnalysiere die Prozessliste. Antworte NUR mit 'SAFE' oder 'SUSPICIOUS'.</s>\n<|user|>\nProzesse: {}\n</s>\n<|assistant|>\nStatus:", 
            text
        );
        
        let tokens = self.tokenizer.encode(prompt, true).map_err(E::msg)?;
        let input = Tensor::new(tokens.get_ids(), &self.device)?.unsqueeze(0)?;
        
        let mut output_tokens = Vec::new();
        let mut current_input = input.clone();
        let mut pos = 0;
        
        for _ in 0..10 { 
            let logits = self.model.forward(&current_input, pos)?;
            let logits = logits.squeeze(0)?; 
            
            let next_token_logits = if logits.rank() == 2 {
                let (seq_len, _) = logits.dims2()?;
                logits.get(seq_len - 1)?
            } else {
                logits
            };

            let next_token = next_token_logits.argmax(0)?.to_scalar::<u32>()?;
            if next_token == 2 { break; } 

            pos += current_input.dim(1)?;
            output_tokens.push(next_token);
            current_input = Tensor::new(&[next_token], &self.device)?.unsqueeze(0)?;
        }
        
        let result = self.tokenizer.decode(&output_tokens, true).map_err(E::msg)?;
        Ok(result.trim().to_string())
    }
}

// --- TEIL 2: ZKP MODUL MIT METRIKEN ---
pub struct ZKPMetrics {
    pub proof_size: usize,
    pub generation_time: std::time::Duration,
    pub risk_score: u64,
    pub context_hash: String,
}

pub struct TardisCircuit {
    pc_gens: PedersenGens,
    bp_gens: BulletproofGens,
}

impl TardisCircuit {
    pub fn boot() -> Self {
        Self { pc_gens: PedersenGens::default(), bp_gens: BulletproofGens::new(64, 1) }
    }
    
    // Hash-Funktion bindet den Beweis an den Inhalt
    fn hash_context(&self, context: &str) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(context.as_bytes());
        hasher.finalize().to_vec()
    }

    pub fn prove_with_metrics(&self, risk_val: u64, context: &str) -> Result<(RangeProof, ZKPMetrics), String> {
        let start = Instant::now();
        
        let context_hash_bytes = self.hash_context(context);
        let context_hash_hex = hex::encode(&context_hash_bytes);

        let mut transcript = Transcript::new(b"EasyEva_v8");
        transcript.append_message(b"process_hash", &context_hash_bytes);

        let mut rng = thread_rng();
        let mut blinding_bytes = [0u8; 32];
        rng.fill_bytes(&mut blinding_bytes);
        let blinding = Scalar::from_bytes_mod_order(blinding_bytes);
        
        let (proof, _) = RangeProof::prove_single(&self.bp_gens, &self.pc_gens, &mut transcript, risk_val, &blinding, 32)
            .map_err(|e| format!("{:?}", e))?;
            
        let duration = start.elapsed();
        
        let metrics = ZKPMetrics {
            proof_size: proof.to_bytes().len(),
            generation_time: duration,
            risk_score: risk_val,
            context_hash: context_hash_hex,
        };
        
        Ok((proof, metrics))
    }
}

fn main() -> Result<()> {
    println!("🔧 EASY-EVA SONIC SCREWDRIVER (v8.0 Platinum)");
    println!("============================================");
    
    let mut ai = LocalAIModule::load("../assets/tinyllama.gguf")?;
    
    let process_list = "termux, bash, rustc, systemd";
    println!("[1] KI Scannt: '{}'", process_list);
    
    let verdict = ai.analyze(process_list)?;
    println!("    -> KI Urteil: '{}'", verdict);
    
    // Mapping: SAFE=0, SUSPICIOUS=1
    let risk_score = if verdict.contains("SAFE") { 0 } else { 1 };
    println!("    -> Risiko-Score: {} (0=Safe, 1=Risk)", risk_score);

    let tardis = TardisCircuit::boot();
    println!("[2] Erstelle kryptographischen Beweis mit Metriken...");
    
    match tardis.prove_with_metrics(risk_score, process_list) {
        Ok((_proof, metrics)) => {
            println!("\n✅ ZERTIFIKAT ERSTELLT.");
            println!("--------------------------------------------");
            println!("📊 ZKP TELEMETRIE:");
            println!("   • Proof Größe:    {} Bytes", metrics.proof_size);
            println!("   • Rechenzeit:     {:.2?}", metrics.generation_time);
            println!("   • Bewiesener Wert: {}", metrics.risk_score);
            println!("   • Daten-Hash:     {}...", &metrics.context_hash[0..16]);
            println!("--------------------------------------------");
        },
        Err(e) => println!("❌ Fehler: {}", e),
    }
    
    Ok(())
}

