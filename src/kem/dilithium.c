#include "../../include/mldsa.h"

int mldsa_keygen(uint8_t *pk, uint8_t *sk) {
    OQS_SIG *sig = OQS_SIG_new("ML-DSA-87");
    if (!sig) return -1;
    int ret = OQS_SIG_keypair(sig, pk, sk);
    OQS_SIG_free(sig);
    return (ret == OQS_SUCCESS) ? 0 : -1;
}

int mldsa_sign(uint8_t *sig_buf, size_t *sig_len, const uint8_t *msg, size_t msg_len, const uint8_t *sk) {
    OQS_SIG *sig = OQS_SIG_new("ML-DSA-87");
    if (!sig) return -1;
    int ret = OQS_SIG_sign(sig, sig_buf, sig_len, msg, msg_len, sk);
    OQS_SIG_free(sig);
    return (ret == OQS_SUCCESS) ? 0 : -1;
}

int mldsa_verify(const uint8_t *msg, size_t msg_len, const uint8_t *sig_buf, size_t sig_len, const uint8_t *pk) {
    OQS_SIG *sig = OQS_SIG_new("ML-DSA-87");
    if (!sig) return -1;
    int ret = OQS_SIG_verify(sig, msg, msg_len, sig_buf, sig_len, pk);
    OQS_SIG_free(sig);
    return (ret == OQS_SUCCESS) ? 0 : -1;
}

