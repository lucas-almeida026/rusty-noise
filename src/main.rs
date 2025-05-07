use clap::Parser;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

mod noise;
use noise::*;

mod cli;
use cli::*;

fn build_audio_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    mut generators: Vec<ControlledNoise>,
    master_volume: f32,
) -> Result<cpal::Stream, cpal::BuildStreamError>
where
    T: cpal::Sample + cpal::FromSample<f32> + cpal::SizedSample,
{
    let channels = config.channels as usize;
    let generator_count = generators.len().max(1) as f32;

    device.build_output_stream(
        config,
        move |data: &mut [T], _| {
            for frame in data.chunks_mut(channels) {
                let mut mixed_sample: f32 = 0.0;
                for ControlledNoise { generator, volume } in generators.iter_mut() {
                    mixed_sample += generator.next_sample() * *volume
                }
                mixed_sample /= generator_count;
                let sample = (mixed_sample * master_volume).clamp(-1.0, 1.0);
                let value: T = T::from_sample(sample);
                for out in frame.iter_mut() {
                    *out = value;
                }
            }
        },
        move |err| {
            eprintln!("Stream error: {}", err);
        },
        None,
    )
}

fn clamp_as_percentage(v: f32) -> f32 {
    if v <= 0.0 {
        0.0
    } else if v >= 100.0 {
        1.0
    } else {
        v / 100.0
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Args::parse();

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device found");
    let config = device.default_output_config()?;

    let master_volume =
        clamp_as_percentage(args.master_volume) / 10.0;
    let generators = vec![
        ControlledNoise::new(NoiseType::Brown, clamp_as_percentage(args.brown) * 33.0),
        ControlledNoise::new(NoiseType::Pink, clamp_as_percentage(args.pink)),
        ControlledNoise::new(NoiseType::White, clamp_as_percentage(args.white)),
        ControlledNoise::new(NoiseType::Blue, clamp_as_percentage(args.blue))
    ];

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => {
            build_audio_stream::<f32>(&device, &config.into(), generators, master_volume)?
        }
        cpal::SampleFormat::I16 => {
            build_audio_stream::<i16>(&device, &config.into(), generators, master_volume)?
        }
        cpal::SampleFormat::U16 => {
            build_audio_stream::<u16>(&device, &config.into(), generators, master_volume)?
        }
        format => panic!("unsuported sample format: {format}"),
    };
    stream.play()?;

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    Ok(())
}
