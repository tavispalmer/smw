#include <stddef.h>
#include <stdint.h>

#include <openssl/sha.h>

void sha1sum(const uint8_t *data, size_t len) {
    SHA_CTX c;
    SHA1_Init(&c);
    SHA1_Final()
}