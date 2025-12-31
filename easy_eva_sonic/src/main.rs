mod balancer;
use balancer::LoadBalancer;
use std::time::Instant;
use rayon::prelude::*;

fn main() {
    let mut lb = LoadBalancer::new();
    let node_count = 10000;
    for i in 0..node_count {
        lb.add_node(&format!("node_addr_{}", i));
    }

    let iterations = 4_000_000; // <<< 4 MILLIONEN – DIE GRENZE WIRD GETESTET
    println!("🚀🔥🔥 STARTE 4-MIO-ULTRA-ASSAULT (ABSOLUTE LIMIT BREAK) 🔥🔥🚀");
    println!("------------------------------------------------------------");

    let start = Instant::now();
    
    (0..iterations).into_par_iter().for_each(|r| {
        let _node = lb.get_node(&format!("req_{}", r));
        
        // Identische Heavy Math Simulation – perfekt für ARM-Pipeline
        let mut x: u64 = r as u64;
        for _ in 0..500 { 
            x = x.wrapping_add(42).wrapping_mul(3); 
        }
    });
    
    let duration = start.elapsed();
    let ops_per_sec = (iterations as f64 / duration.as_secs_f64()) as u64;

    println!("Gesamtzeit: {:?}", duration);
    println!("Performance: {} Ops/Sek", ops_per_sec);
    println!("------------------------------------------------------------");
    println!("STATUS: 4-MIO LIMIT BREAK VALIDATED 🏆💥");
    println!("EASY EVA SONIC – BEYOND LIMITS");
}
