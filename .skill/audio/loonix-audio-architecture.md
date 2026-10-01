# Loonix Audio Engine Design

Engine harus stabil, low latency, dan bebas glitch.

## Engine pipeline

File
→ FFmpeg demux
→ FFmpeg decode
→ PCM buffer
→ DSP rack
→ PipeWire output

Engine hanya decode audio.
Semua processing dilakukan setelah decode.

## Thread architecture

Minimal 3 thread:

1. decode thread
2. audio output thread
3. control thread

Decode thread:
mengambil frame dari FFmpeg.

Audio thread:
mengirim buffer ke PipeWire.

Control thread:
handle play pause seek dll.

## Buffer strategy

Gunakan ring buffer.
Ukuran buffer ideal:
100ms – 300ms audio

Contoh:
48000 Hz stereo
buffer ≈ 9600 – 28800 samples
Ini mencegah underrun.

## Anti glitch rules

Jangan:

- decode di audio thread
- blocking di audio callback
- melakukan seek di thread audio

Semua operasi berat dilakukan di decode thread.

## Clock system

Audio device adalah master clock.
Bukan UI.
Bukan decoder.

## Format internal

Gunakan PCM float32.
Ini lebih stabil untuk DSP processing.

## Error handling

Jika decode gagal:

- skip frame
- jangan stop engine
