#include "wrapper.h"
#include <sfc/sfc.hpp>
#include <ananke/heuristics/super-famicom.hpp>

#define AUDIO_SAMPLE_RATE 32040.5
#define VIDEO_REFRESH_RATE_PAL (21281370.0 / 425568.0)

const uint8 iplrom[64] = {
    /*ffc0*/ 0xcd, 0xef,       // mov   x,#$ef
    /*ffc2*/ 0xbd,             // mov   sp,x
    /*ffc3*/ 0xe8, 0x00,       // mov   a,#$00
    /*ffc5*/ 0xc6,             // mov   (x),a
    /*ffc6*/ 0x1d,             // dec   x
    /*ffc7*/ 0xd0, 0xfc,       // bne   $ffc5
    /*ffc9*/ 0x8f, 0xaa, 0xf4, // mov   $f4,#$aa
    /*ffcc*/ 0x8f, 0xbb, 0xf5, // mov   $f5,#$bb
    /*ffcf*/ 0x78, 0xcc, 0xf4, // cmp   $f4,#$cc
    /*ffd2*/ 0xd0, 0xfb,       // bne   $ffcf
    /*ffd4*/ 0x2f, 0x19,       // bra   $ffef
    /*ffd6*/ 0xeb, 0xf4,       // mov   y,$f4
    /*ffd8*/ 0xd0, 0xfc,       // bne   $ffd6
    /*ffda*/ 0x7e, 0xf4,       // cmp   y,$f4
    /*ffdc*/ 0xd0, 0x0b,       // bne   $ffe9
    /*ffde*/ 0xe4, 0xf5,       // mov   a,$f5
    /*ffe0*/ 0xcb, 0xf4,       // mov   $f4,y
    /*ffe2*/ 0xd7, 0x00,       // mov   ($00)+y,a
    /*ffe4*/ 0xfc,             // inc   y
    /*ffe5*/ 0xd0, 0xf3,       // bne   $ffda
    /*ffe7*/ 0xab, 0x01,       // inc   $01
    /*ffe9*/ 0x10, 0xef,       // bpl   $ffda
    /*ffeb*/ 0x7e, 0xf4,       // cmp   y,$f4
    /*ffed*/ 0x10, 0xeb,       // bpl   $ffda
    /*ffef*/ 0xba, 0xf6,       // movw  ya,$f6
    /*fff1*/ 0xda, 0x00,       // movw  $00,ya
    /*fff3*/ 0xba, 0xf4,       // movw  ya,$f4
    /*fff5*/ 0xc4, 0xf4,       // mov   $f4,a
    /*fff7*/ 0xdd,             // mov   a,y
    /*fff8*/ 0x5d,             // mov   x,a
    /*fff9*/ 0xd0, 0xdb,       // bne   $ffd6
    /*fffb*/ 0x1f, 0x00, 0x00, // jmp   ($0000+x)
    /*fffe*/ 0xc0, 0xff        // reset vector location ($ffc0)
};

struct Callbacks : Emulator::Interface::Bind {
  void *user;
  const SfcCallbacks *clbk;
  const uint8_t *buf;
  size_t len;

  Emulator::Interface *iface;

  uint8_t *sram;
  size_t sram_size;

  uint32_t video_buffer[512 * 480];

  vector<int16_t> sample_buf;
  size_t sample_buf_pos;

  void videoRefresh(const uint32_t *palette, const uint32_t *data,
                    unsigned pitch, unsigned width, unsigned height) override {
    uint32_t *ptr = video_buffer;
    for (size_t y = 0; y < height; y++, data += pitch >> 2, ptr += width) {
      for (size_t x = 0; x < width; x++) {
        ptr[x] = palette[data[x]];
      }
    }
    clbk->video_refresh(user, reinterpret_cast<const uint8_t *>(video_buffer),
                        width, height, width * sizeof(uint32_t));
  }

  void audioSample(int16_t left, int16_t right) override {
    unsigned buf_capacity = sample_buf.capacity();
    if (buf_capacity - sample_buf_pos < 2) {
      unsigned new_size = ((buf_capacity + 2) << 1) - ((buf_capacity + 2) >> 1);
      sample_buf.resize(new_size);
    }
    sample_buf[sample_buf_pos++] = left;
    sample_buf[sample_buf_pos++] = right;
  }

  void loadRequest(unsigned id, string p) override {
    switch (id) {
    case SuperFamicom::ID::IPLROM: {
      memorystream stream(iplrom, sizeof(iplrom));
      iface->load(id, stream);
    } break;

    case SuperFamicom::ID::Manifest: {
      string xmlrom = SuperFamicomCartridge(buf, len).markup;
      memorystream stream((const uint8_t *)(const char *)xmlrom,
                          xmlrom.length());
      iface->load(id, stream);
    } break;

    case SuperFamicom::ID::ROM: {
      memorystream stream(buf, len);
      iface->load(id, stream);
    } break;

    case SuperFamicom::ID::RAM: {
      sram = SuperFamicom::cartridge.ram.data();
      sram_size = SuperFamicom::cartridge.ram.size();
    } break;

    default:
      fprintf(stderr, "unknown request type: %d!\n", id);
      break;
    }
  }

  uint32_t videoColor(unsigned, uint16_t, uint16_t r, uint16_t g,
                      uint16_t b) override {
    r >>= 8;
    g >>= 8;
    b >>= 8;
    return (r << 16) | (g << 8) | (b << 0);
  }

  void notify(string text) override {
    fprintf(stderr, "%s\n", (const char *)text);
  }
};

static Callbacks sfc_bind;

struct Interface : public SuperFamicom::Interface {
  Interface() { bind = &sfc_bind; }
};

static Interface sfc_interface;

struct UserGuard {
  UserGuard(void *user) { sfc_bind.user = user; }
  ~UserGuard() { sfc_bind.user = nullptr; }
};

void sfc_init(void *user, const SfcCallbacks *clbk, const uint8_t *buf,
              size_t len) {
  UserGuard userGuard(user);
  sfc_bind.clbk = clbk;
  sfc_bind.buf = buf;
  sfc_bind.len = len;

  sfc_bind.iface = &sfc_interface;
  SuperFamicom::interface = &sfc_interface;

  SuperFamicom::video.generate_palette(
      Emulator::Interface::PaletteMode::Standard);

  sfc_bind.sample_buf.resize(
      ((unsigned)(AUDIO_SAMPLE_RATE / VIDEO_REFRESH_RATE_PAL) + 1) << 1);
  sfc_bind.sample_buf_pos = 0;

  SuperFamicom::system.init();
  SuperFamicom::input.connect(SuperFamicom::Controller::Port1,
                              SuperFamicom::Input::Device::Joypad);
  SuperFamicom::input.connect(SuperFamicom::Controller::Port2,
                              SuperFamicom::Input::Device::Joypad);

  sfc_bind.iface->load(SuperFamicom::ID::SuperFamicom);
  SuperFamicom::system.power();
}

void sfc_iter(void *user) {
  UserGuard userGuard(user);
  SuperFamicom::system.run();
  if (sfc_bind.sample_buf_pos) {
    sfc_bind.clbk->audio_sample_batch(sfc_bind.user, sfc_bind.sample_buf.data(),
                                      sfc_bind.sample_buf_pos >> 1);
    sfc_bind.sample_buf_pos = 0;
  }
}

void sfc_quit(void *user) {
  UserGuard userGuard(user);
  sfc_bind.iface->save();
  SuperFamicom::cartridge.unload();

  SuperFamicom::system.term();
}
