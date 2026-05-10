#ifndef _WRAPPER_H
#define _WRAPPER_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>
#include <stdint.h>

typedef struct {
  void (*video_refresh)(void *user, const void *data, unsigned width,
                        unsigned height, size_t pitch);
  size_t (*audio_sample_batch)(void *user, const int16_t *data, size_t frames);
} SfcCallbacks;

void sfc_init(void *user, const SfcCallbacks *clbk, const uint8_t *buf,
              size_t len);
void sfc_iter(void *user);
void sfc_quit(void *user);

#ifdef __cplusplus
}
#endif

#endif
