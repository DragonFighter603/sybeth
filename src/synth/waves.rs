use std::f32::consts::PI;
use std::sync::MutexGuard;
use crate::synth::Synth;
use crate::synth::synth_source::SynthInput;

pub(crate) struct SineSynth;

impl Synth for SineSynth {
    fn get(&mut self, time: usize, input: MutexGuard<SynthInput>) -> f32 {
        let x = 2.0 * input.freq * time as f32 / 44000.0;
        f32::sin( PI * x)
    }
}

pub(crate) struct SquareSynth;

impl Synth for SquareSynth {
    fn get(&mut self, time: usize, input: MutexGuard<SynthInput>) -> f32 {
        let x = 2.0 * input.freq * time as f32 / 44000.0;
        f32::signum(x % 2.0 - 1.0)
    }
}

pub(crate) struct TriangleSynth;

impl Synth for TriangleSynth {
    fn get(&mut self, time: usize, input: MutexGuard<SynthInput>) -> f32 {
        let x = 2.0 * input.freq * time as f32 / 44000.0;
        f32::abs(x % 2.0 - 1.0) * 2.0 - 1.0
    }
}

pub(crate) struct SawSynth;

impl Synth for SawSynth {
    fn get(&mut self, time: usize, input: MutexGuard<SynthInput>) -> f32 {
        let x = 2.0 * input.freq * time as f32 / 44000.0;
        x % 2.0 - 1.0
    }
}