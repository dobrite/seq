#![cfg_attr(not(test), no_std)]

pub use output::{Config as OutputConfig, Density, Frac, Length, OutputType, Prob, Pwm, Rate};

pub use crate::seq::Seq;

mod maff;
mod output;
mod seq;
mod tick;
