use anyhow::{Error as E, Result};
use candle_transformers::models::quantized_llama::ModelWeights as QGemma;
use candle_core::{Device, Tensor};
use candle_core::quantized::gguf_file;
use tokenizers::Tokenizer;
use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};
use curve25519_dalek_ng::scalar::Scalar;
use merlin::Transcript;
use rand::{thread_rng, RngCore};

// --- TEIL 1: KI MODUL ---
pub struct LocalAIModule {
    model: QGemma,
    tokenizer: Tokenizer,
    device: Device,
}

impl LocalAIModule {
    pub fn load(model_path: &str) -> Result<Self> {
        println!("🚀 [AI-Core] Lade TinyLlama (Gold Master)...");
        let device = Device::Cpu;
        let mut file = std::fs::File::open(model_path)?;
        
        let content = gguf_file::Content::read(&mut file)?;
        let model = QGemma::from_gguf(content, &mut file, &device)?;
        
        let tokenizer = Tokenizer::from_file("../assets/tokenizer.json").map_err(E::msg)?;
        Ok(Self { model, tokenizer, device })
    }

    pub fn analyze(&mut self, text: &str) -> Result<String> {
        // SYSTEM-PROMPT: Zwingt die KI, ein Polizist zu sein
        let prompt = format!(
            "<|system|>\nAnalysiere die Prozessliste. Antworte NUR mit 'SAFE' oder 'SUSPICIOUS'.</s>\n<|user|>\nProzesse: {}\n</s>\n<|assistant|>\nStatus:", 
            text
        );
        
        let tokens = self.tokenizer.encode(prompt, true).map_err(E::msg)?;
        let input = Tensor::new(tokens.get_ids(), &self.device)?.unsqueeze(0)?;
        
        let mut output_tokens = Vec::new();
        let mut current_input = input.clone();
        let mut pos = 0;
        
        // Wir brauchen nur maximal 10 Tokens für ein kurzes "SAFE"
        for _ in 0..10 { 
            let logits = self.model.forward(&current_input, pos)?;
            let logits = logits.squeeze(0)?; 

            // SAFETY CHECK: Verhindert Absturz bei Dimensionen
            let next_token_logits = if logits.rank() == 2 {
                let (seq_len, _) = logits.dims2()?;
                logits.get(seq_len - 1)?
            } else {
                logits
            };

            let next_token = next_token_logits.argmax(0)?.to_scalar::<u32>()?;
            
            // Stoppen, wenn das Satzende-Token (2) kommt
            if next_token == 2 { break; }

            pos += current_input.dim(1)?;
            output_tokens.push(next_token);
            current_input = Tensor::new(&[next_token], &self.device)?.unsqueeze(0)?;
        }
        
        let result = self.tokenizer.decode(&output_tokens, true).map_err(E::msg)?;
        // Wir säubern das Ergebnis, falls die KI doch noch Tags mitliefert
        Ok(result.trim().to_string())
    }
}

// --- TEIL 2: ZKP MODUL ---
pub struct TardisCircuit {
    pc_gens: PedersenGens,
    bp_gens: BulletproofGens,
}

impl TardisCircuit {
    pub fn boot() -> Self {
        Self { pc_gens: PedersenGens::default(), bp_gens: BulletproofGens::new(64, 1) }
    }
    
    pub fn prove_with_context(&self, val: u64, context: &str) -> Result<RangeProof, String> {
        let mut transcript = Transcript::new(b"EasyEva_v1");
        transcript.append_message(b"ai_context", context.as_bytes());
        
        let mut rng = thread_rng();
        let mut blinding_bytes = [0u8; 32];
        rng.fill_bytes(&mut blinding_bytes);
        let blinding = Scalar::from_bytes_mod_order(blinding_bytes);
        
        let (proof, _) = RangeProof::prove_single(&self.bp_gens, &self.pc_gens, &mut transcript, val, &blinding, 32)
            .map_err(|e| format!("{:?}", e))?;
        Ok(proof)
    }
}

fn main() -> Result<()> {
    println!("🔧 EASY-EVA SONIC SCREWDRIVER (Gold Master)");
    
    let mut ai = LocalAIModule::load("../assets/tinyllama.gguf")?;
    
    let process_list = "termux, bash, rustc";
    println!("[1] KI Analysiert Prozesse: '{}'", process_list);
    
    let verdict = ai.analyze(process_list)?;
    println!("    -> KI Urteil: {}", verdict);
    
    let tardis = TardisCircuit::boot();
    println!("[2] Erstelle kryptographischen Beweis...");
    
    match tardis.prove_with_context(3, &verdict) {
        Ok(_) => println!("✅ ZERTIFIKAT ERSTELLT (KI + ZKP)."),
        Err(e) => println!("❌ Fehler: {}", e),
    }
    
    Ok(())
}

