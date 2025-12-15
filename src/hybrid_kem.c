#include "hybrid_kem.h"
#include <openssl/rand.h>
#include <string.h>

const char* hybrid_kem_version(void) {
    return "0.1.0-termux";
}

int hybrid_kem_keypair(uint8_t pk[], uint8_t sk[]) {
    if (!pk || !sk) return HYBRID_ERROR;
    
    // Einfache Zufallsdaten mit OpenSSL
    if (RAND_bytes(pk, HYBRID_KEM_PUBLIC_KEY_SIZE) != 1) {
        return HYBRID_ERROR;
    }
    
    if (RAND_bytes(sk, HYBRID_KEM_PRIVATE_KEY_SIZE) != 1) {
        return HYBRID_ERROR;
    }
    
    return HYBRID_SUCCESS;
}
