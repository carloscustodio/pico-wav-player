# Raspberry Pi Pico WAV Player

A Rust embedded project that plays WAV audio data on GPIO pin 0 of a Raspberry Pi Pico.

## Hardware Setup

- Raspberry Pi Pico
- Audio output on GPIO 0
- Connect GPIO 0 to a speaker/amplifier circuit (use appropriate resistors and capacitors)
- Recommended: Add a low-pass filter circuit between GPIO 0 and speaker

## Building

```bash
# Install Rust embedded target
rustup target add thumbv6m-none-eabi

# Install probe-rs for flashing (optional)
cargo install probe-rs --features cli

# Build the project
cargo build --release
```

## Flashing

### Using probe-rs (recommended):
```bash
cargo run --release
```

### Using elf2uf2-rs:
```bash
cargo install elf2uf2-rs
elf2uf2-rs target/thumbv6m-none-eabi/release/pico-wav-player
# Copy the generated .uf2 file to the Pico in bootloader mode
```

## Customizing Audio

Replace the WAV_DATA in `src/wav_data.rs` with your own audio samples:
- Use 8-bit unsigned mono samples (0-255)
- Adjust SAMPLE_RATE as needed
- Keep samples in flash memory or use external storage for larger files

## Circuit Notes

For basic audio output:
- GPIO 0 → 1kΩ resistor → RC low-pass filter → speaker/amplifier
- Add a 100µF capacitor in series to block DC
- Consider using a proper audio amplifier for better volume

## Limitations

This implementation uses simple bit-banging for audio output. For better quality audio:
- Use PWM peripheral for true analog output
- Implement proper sigma-delta modulation
- Use I2S for high-quality audio output
