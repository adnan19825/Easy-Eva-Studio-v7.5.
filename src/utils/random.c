#include <openssl/rand.h>
int get_random_bytes(void *buf, size_t len) {
    return RAND_bytes(buf, (int)len) == 1;
}
