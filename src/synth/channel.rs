use std::sync::{Arc, Mutex};
use rodio::Sink;
use crate::synth::synth_source::{SynthInput};

pub(crate) struct SynthChannel {
    pub(crate) name: &'static str,
    pub(crate) sink: Sink,
    pub(crate) input: Arc<Mutex<SynthInput>>
}