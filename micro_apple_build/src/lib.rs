use std::{num::NonZero, path::PathBuf};

use thiserror::Error;

const OUTPUT_SIZE: usize = 5;
const OUTPUT_FRAME_LENGTH: usize = OUTPUT_SIZE * OUTPUT_SIZE;

pub struct MiniFrame(pub [u8; OUTPUT_FRAME_LENGTH]);

impl MiniFrame {
    pub fn pretty_string(&self) -> String {
        let mut built = String::with_capacity(4 * OUTPUT_SIZE * OUTPUT_SIZE);

        for i in 0..OUTPUT_FRAME_LENGTH {
            let pixel_value = self.0[i];
            built.push_str(&format!("{pixel_value:0>3}"));

            if i % OUTPUT_SIZE < OUTPUT_SIZE - 1 {
                built.push(' ');
            } else {
                built.push('\n');
            }
        }

        built
    }
}

pub fn generate_mini_frames(path: &PathBuf) -> Result<Vec<MiniFrame>, GenerationError> {
    let mut output_frames = Vec::new();

    let mut input = ffmpeg_next::format::input(path)?;
    let input_stream = input
        .streams()
        .best(ffmpeg_next::media::Type::Video)
        .ok_or(ffmpeg_next::Error::StreamNotFound)?;
    let video_stream_index = input_stream.index();

    let mut context_decoder =
        ffmpeg_next::codec::Context::from_parameters(input_stream.parameters())?;
    context_decoder.set_threading(ffmpeg_next::threading::Config {
        kind: ffmpeg_next::threading::Type::Frame,
        count: std::thread::available_parallelism()
            .unwrap_or(NonZero::new(1).unwrap())
            .get()
            .min(16),
    });
    let mut decoder = context_decoder.decoder().video()?;
    let (width, height) = (decoder.width(), decoder.height());

    let mut scaler = ffmpeg_next::software::scaling::Context::get(
        decoder.format(),
        width,
        height,
        ffmpeg_next::format::Pixel::GRAY8,
        OUTPUT_SIZE as u32,
        OUTPUT_SIZE as u32,
        ffmpeg_next::software::scaling::Flags::BICUBIC,
    )?;

    let mut decoded = ffmpeg_next::util::frame::Video::new(decoder.format(), width, height);
    let mut scaled = ffmpeg_next::frame::Video::new(
        ffmpeg_next::format::Pixel::GRAY8,
        OUTPUT_SIZE as u32,
        OUTPUT_SIZE as u32,
    );

    for (stream, packet) in input.packets() {
        if stream.index() != video_stream_index {
            continue;
        }

        decoder.send_packet(&packet)?;
        while decoder.receive_frame(&mut decoded).is_ok() {
            scaler.run(&decoded, &mut scaled)?;
            let mut output = [0u8; OUTPUT_FRAME_LENGTH];

            for row in 0..OUTPUT_SIZE {
                let row_index = row * OUTPUT_SIZE;
                let stride = scaled.stride(0);
                let strided_index = stride * row;
                output[row_index..row_index + OUTPUT_SIZE]
                    .copy_from_slice(&scaled.data(0)[strided_index..strided_index + OUTPUT_SIZE]);
            }

            output_frames.push(MiniFrame(output));
        }
    }

    Ok(output_frames)
}

#[derive(Debug, Error)]
pub enum GenerationError {
    #[error("error from ffmpeg: {0}")]
    Ffmpeg(#[from] ffmpeg_next::Error),
}

pub fn init() -> Result<(), ffmpeg_next::Error> {
    ffmpeg_next::init()
}

#[derive(Debug, Error)]
#[error("failed to init ffmpeg: {0}")]
pub struct InitError(#[from] ffmpeg_next::Error);
