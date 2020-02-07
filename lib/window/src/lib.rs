
pub mod event;

#[cfg(target_arch = "x86_64")]
pub mod glutin_window;
#[cfg(target_arch = "x86_64")]
pub use crate::glutin_window::*;

#[cfg(target_arch = "wasm32")]
pub mod canvas;
#[cfg(target_arch = "wasm32")]
pub use crate::canvas::*;

pub struct FrameInput {
    pub events: Vec<event::Event>,
    pub elapsed_time: f64,
    pub screen_width: usize,
    pub screen_height: usize
}