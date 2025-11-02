#![no_std]
#![no_main]

mod wav_data;

use cortex_m::delay::Delay;
use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;
use rp2040_hal::{clocks::init_clocks_and_plls, pac, watchdog::Watchdog, Clock, Sio};

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

#[entry]
fn main() -> ! {
    // Setup peripherals
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // Configure clocks (12 MHz external crystal on Pico)
    let clocks = init_clocks_and_plls(
        12_000_000u32,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Setup GPIO pins
    let pins = rp2040_hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Configure GPIO0 as output for audio (PWM-like output using bit-banging)
    let mut audio_pin = pins.gpio0.into_push_pull_output();

    // Setup delay for timing
    let sys_freq = clocks.system_clock.freq().to_Hz();
    let mut delay = Delay::new(core.SYST, sys_freq);

    // Calculate delay between samples in microseconds
    let sample_delay_us = 1_000_000 / wav_data::SAMPLE_RATE;

    loop {
        // Play WAV data
        for &sample in wav_data::WAV_DATA.iter() {
            // Simple 1-bit audio: output high/low based on sample value
            // For better quality, implement PWM, but this demonstrates the concept
            if sample > 128 {
                audio_pin.set_high().unwrap();
            } else {
                audio_pin.set_low().unwrap();
            }

            // Wait for the sample period
            delay.delay_us(sample_delay_us);
        }

        // Small pause between loops
        delay.delay_ms(100);
    }
}
