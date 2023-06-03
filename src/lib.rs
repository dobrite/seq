#![cfg_attr(not(test), no_std)]

pub use output::{
    euclid, Config as OutputConfig, Density, Frac, Length, OutputType, Prob, Pwm, Rate,
};

pub use crate::seq::Seq;

mod math;
mod output;
mod seq;
mod tick;
