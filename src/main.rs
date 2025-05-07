use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

mod noise;
use noise::*;


fn build_audio_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    mut generators: Vec<ControlledNoise>,
	master_volume: f32
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

// fn play_white_noise(volume: f32) -> Result<(), Box<dyn std::error::Error>> {

// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device found");
    let config = device.default_output_config()?;

    let volume = 0.05;
    let generators = vec![
		ControlledNoise::new(NoiseType::Brown, 1.0),
		ControlledNoise::new(NoiseType::Pink, 0.1),
		ControlledNoise::new(NoiseType::White, 0.01)
	];

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => {
            build_audio_stream::<f32>(&device, &config.into(), generators, volume)?
        }
        cpal::SampleFormat::I16 => {
            build_audio_stream::<i16>(&device, &config.into(), generators, volume)?
        }
        cpal::SampleFormat::U16 => {
            build_audio_stream::<u16>(&device, &config.into(), generators, volume)?
        }
        format => panic!("unsuported sample format: {format}"),
    };

    stream.play()?;
    println!(
        "White noise playing at volume {}. Press Ctrl+C to stop.",
        volume
    );

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
