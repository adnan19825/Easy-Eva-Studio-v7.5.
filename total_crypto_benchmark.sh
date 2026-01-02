#!/bin/bash
clear
echo "===================================================="
echo "   EASY-EVA v7.5: HYBRID PQC & FHE BENCHMARK"
echo "===================================================="

# 1. PQC BENCHMARK (Transport Layer)
echo -e "\n[1/2] Messung: Hybrid Post-Quantum Handshake (P-384 + ML-KEM-1024)"
echo "----------------------------------------------------"
cd ~/hybrid-kem/build
./benchmark_hybrid | grep -E "KeyGen|Encaps|Decaps"

# 2. FHE BENCHMARK (Processing Layer)
echo -e "\n[2/2] Messung: Homomorphe Multiplikation (3-Bit TFHE)"
echo "----------------------------------------------------"
cd ~/fhe-workspace
# Wir nutzen die bereits kompilierte release-Binary
target/release/fhe-main | grep -E "Dauer|Resultat"

echo -e "\n===================================================="
echo "   BENCHMARK ABGESCHLOSSEN"
echo "===================================================="
