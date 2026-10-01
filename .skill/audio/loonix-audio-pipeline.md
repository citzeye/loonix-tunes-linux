FILE
↓
FFmpeg Decode Engine
↓
PCM Buffer
↓
Effect Rack

- EQ
- Compressor
- Reverb
- Surround
- Crystalizer
  ↓
  VST3 Host (optional)
  ↓
  PipeWire Output
  ↓
  Sound Card

Prinsip penting :
Engine hanya decode
Engine tidak menjalankan efek
Semua efek berada di post-processing chain
