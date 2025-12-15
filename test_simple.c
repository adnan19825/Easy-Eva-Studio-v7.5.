#include <stdio.h>
#include <stdint.h>
#include <openssl/rand.h>

#define PK_SIZE 1568
#define SK_SIZE 3168

int main() {
    uint8_t pk[PK_SIZE];
    uint8_t sk[SK_SIZE];
    
    printf("Testing OpenSSL in Termux...\n");
    
    if (RAND_bytes(pk, 16) == 1) {
        printf("OpenSSL RAND working! Random bytes: ");
        for (int i = 0; i < 8; i++) {
            printf("%02x", pk[i]);
        }
        printf("\nSuccess!\n");
    } else {
        printf("Failed to generate random bytes\n");
    }
    
    return 0;
}
