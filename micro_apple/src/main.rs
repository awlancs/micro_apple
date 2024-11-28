#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_halt as _;

use cortex_m_rt::entry;
use microbit::{display::blocking::Display, hal::{Clocks, Timer}, Board};
use ringbuffer::RingBuffer;

#[entry]
fn main() -> ! {
    let rle_encoded = include_bytes!("../../bad_apple_rle.bin");

    if let Some(board) = Board::take() {
        let mut timer = Timer::new(board.TIMER0);
        let clocks = Clocks::new(board.CLOCK);
        clocks.start_lfclk();
        let mut display = Display::new(board.display_pins);
        display.set_delay_ms(0);

        let mut index_into_rle = 0;

        let mut buffer: ringbuffer::ConstGenericRingBuffer<u8, 512> =
            ringbuffer::ConstGenericRingBuffer::new();
        let mut screen_buffer = [[0u8; 5]; 5];

        loop {
            if index_into_rle >= rle_encoded.len() {
                break;
            }

            let byte = rle_encoded[index_into_rle];
            let repeats = byte >> 1;
            let value = byte & 0b1;

            for _ in 0..repeats {
                buffer.push(value);
            }

            index_into_rle += 1;

            while buffer.len() > 25 {
                for screen_index in 0..25 {
                    screen_buffer[screen_index / 5][screen_index % 5] = buffer.dequeue().unwrap();
                }

                display.show(&mut timer, screen_buffer, (1.0 / 30.0 * 1e3) as u32);
            }
        }
    }

    loop {
        continue;
    }
}
