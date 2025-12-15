#ifndef HYBRID_KEM_H
#define HYBRID_KEM_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// Konstanten
#define HYBRID_KEM_PUBLIC_KEY_SIZE 1568
#define HYBRID_KEM_PRIVATE_KEY_SIZE 3168
#define HYBRID_KEM_SHARED_SECRET_SIZE 32

// Fehlercodes
typedef enum {
    HYBRID_SUCCESS = 0,
    HYBRID_ERROR = 1
} hybrid_error_t;

// API Funktionen
const char* hybrid_kem_version(void);

int hybrid_kem_keypair(
    uint8_t pk[HYBRID_KEM_PUBLIC_KEY_SIZE],
    uint8_t sk[HYBRID_KEM_PRIVATE_KEY_SIZE]
);

#ifdef __cplusplus
}
#endif

#endif
