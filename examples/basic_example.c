#include "hybrid_kem.h"
#include <stdio.h>
#include <stdlib.h>

int main(void) {
    printf("Hybrid KEM Version: %s\n", hybrid_kem_version());
    
    uint8_t pk[HYBRID_KEM_PUBLIC_KEY_SIZE];
    uint8_t sk[HYBRID_KEM_PRIVATE_KEY_SIZE];
    
    int result = hybrid_kem_keypair(pk, sk);
    
    if (result == HYBRID_SUCCESS) {
        printf("✅ Schlüsselpaar erfolgreich generiert!\n");
        printf("Public Key (erste 16 Bytes): ");
        for (int i = 0; i < 16; i++) {
            printf("%02x", pk[i]);
        }
        printf("\n");
    } else {
        printf("❌ Fehler bei der Generierung\n");
    }
    
    return 0;
}
