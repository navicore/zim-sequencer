use rodio::{OutputStream, Sink, Source};
use std::time::Duration;
use crate::music_theory::Note;

pub struct Synth {
    _stream: OutputStream,
    sink: Sink,
}

impl Synth {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        
        Ok(Synth {
            _stream: stream,
            sink,
        })
    }
    
    pub fn play_notes(&self, notes: &[Note], duration_ms: u64) {
        // Don't stop previous sounds - allow overlapping/sequential playback
        // self.sink.stop();
        
        // Create a mixed signal for all notes (additive synthesis)
        let duration = Duration::from_millis(duration_ms);
        let sample_rate = 44100;
        let total_samples = (sample_rate as f32 * duration.as_secs_f32()) as usize;
        
        // Generate samples
        let mut samples = vec![0.0f32; total_samples];
        
        for note in notes {
            let freq = note.to_frequency();
            let amplitude = 0.3 / notes.len().max(1) as f32; // Scale amplitude by number of notes
            
            for (i, sample) in samples.iter_mut().enumerate() {
                let t = i as f32 / sample_rate as f32;
                *sample += amplitude * (2.0 * std::f32::consts::PI * freq * t).sin();
            }
        }
        
        // Convert to source and play
        let source = SamplesBuffer {
            samples,
            sample_rate,
            current: 0,
        };
        
        self.sink.append(source);
        self.sink.play();
    }
    
    pub fn stop(&self) {
        self.sink.stop();
    }
}

struct SamplesBuffer {
    samples: Vec<f32>,
    sample_rate: u32,
    current: usize,
}

impl Iterator for SamplesBuffer {
    type Item = f32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.samples.len() {
            let sample = self.samples[self.current];
            self.current += 1;
            Some(sample)
        } else {
            None
        }
    }
}

impl Source for SamplesBuffer {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    
    fn channels(&self) -> u16 {
        1
    }
    
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
    
    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs_f32(
            self.samples.len() as f32 / self.sample_rate as f32
        ))
    }
}