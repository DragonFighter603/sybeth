use std::sync::{Arc, Mutex};
use std::time::Duration;
use rodio::Source;
use crate::synth::{Synth};
use crate::synth::waves::SAMPLE_RATE;

pub(crate) struct SynthInput {
    pub(crate) playing: bool,
    pub(crate) freq: f32
}

pub(crate) struct SynthSource {
    input: Arc<Mutex<SynthInput>>,
    provider: Box<dyn Synth>,
    time: usize,
}

impl SynthSource {
    pub fn create(synth: Box<dyn Synth>) -> (SynthSource, Arc<Mutex<SynthInput>>) {
        let i = Arc::new(Mutex::new(SynthInput {
            playing: false,
            freq: 0.0,
        }));

        let s = Self {
            input: i.clone(),
            time: 0,
            provider: synth
        };

        (s, i)
    }
}

unsafe impl Send for SynthSource {}

impl Iterator for SynthSource {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.time = self.time.wrapping_add(1);
        let input = self.input.lock().unwrap();
        if input.playing{
            Some(self.provider.get(self.time, input))
        } else {
            self.time = 0;
            Some(0.0)
        }
    }
}

impl Source for SynthSource {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE as u32
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
