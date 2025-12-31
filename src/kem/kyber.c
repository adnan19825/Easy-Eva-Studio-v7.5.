#include "kyber.h"
#include <oqs/oqs.h>
#include <stdlib.h>
#include <stdio.h>

kyber_ctx_t* kyber_new(kyber_security_level_t level) {
    kyber_ctx_t *wrapper = malloc(sizeof(kyber_ctx_t));
    if (!wrapper) return NULL;
    wrapper->level = level;

    const char *alg_name = "ML-KEM-1024";
    printf("[DEBUG] Initiiere %s...\n", alg_name);
    
    OQS_KEM *oqs_kem = OQS_KEM_new(alg_name);
    if (!oqs_kem) {
        printf("[ERROR] OQS_KEM_new failed\n");
        free(wrapper);
        return NULL;
    }
    wrapper->ctx = (void*)oqs_kem;
    return wrapper;
}

void kyber_free(kyber_ctx_t *ctx) {
    if (ctx) {
        if (ctx->ctx) OQS_KEM_free((OQS_KEM*)ctx->ctx);
        free(ctx);
    }
}

int kyber_keygen(kyber_ctx_t *ctx, uint8_t *pk, size_t pk_len, uint8_t *sk, size_t sk_len) {
    if (!ctx || !ctx->ctx) return -1;
    return OQS_KEM_keypair((OQS_KEM*)ctx->ctx, pk, sk) == OQS_SUCCESS ? 0 : -1;
}

int kyber_encapsulate(kyber_ctx_t *ctx, const uint8_t *pk, size_t pk_len, uint8_t *ct, size_t ct_len, uint8_t *ss, size_t ss_len) {
    if (!ctx || !ctx->ctx) return -1;
    return OQS_KEM_encaps((OQS_KEM*)ctx->ctx, ct, ss, pk) == OQS_SUCCESS ? 0 : -1;
}

int kyber_decapsulate(kyber_ctx_t *ctx, const uint8_t *sk, size_t sk_len, const uint8_t *ct, size_t ct_len, uint8_t *ss, size_t ss_len) {
    if (!ctx || !ctx->ctx) return -1;
    return OQS_KEM_decaps((OQS_KEM*)ctx->ctx, ss, ct, sk) == OQS_SUCCESS ? 0 : -1;
}
