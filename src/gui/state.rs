use crossbeam::atomic::AtomicCell;
use nih_plug::params::persist::PersistentField;
use serde::{Deserialize, Serialize};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

fn empty_size_fn() -> Box<dyn Fn() -> (u32, u32) + Send + Sync> {
    Box::new(|| (0, 0))
}

#[derive(Serialize, Deserialize)]
pub struct ViziaState {
    #[serde(skip, default = "empty_size_fn")]
    size_fn: Box<dyn Fn() -> (u32, u32) + Send + Sync>,
    /// A scale factor that should be applied to `size` separate from from any system HiDPI scaling.
    /// This can be used to allow GUIs to be scaled uniformly.
    #[serde(with = "nih_plug::params::persist::serialize_atomic_cell")]
    scale_factor: AtomicCell<f64>,
    /// Whether the editor's window is currently open.
    #[serde(skip)]
    open: AtomicBool,
}

impl<'a> PersistentField<'a, ViziaState> for Arc<ViziaState> {
    fn set(&self, new_value: ViziaState) {
        self.scale_factor.store(new_value.scale_factor.load());
    }

    fn map<F, R>(&self, f: F) -> R
    where
        F: Fn(&ViziaState) -> R,
    {
        f(self)
    }
}

impl ViziaState {
    pub fn new(size_fn: impl Fn() -> (u32, u32) + Send + Sync + 'static) -> Arc<ViziaState> {
        Arc::new(ViziaState {
            size_fn: Box::new(size_fn),
            scale_factor: AtomicCell::new(1.0),
            open: AtomicBool::new(false),
        })
    }
    pub fn is_open(&self) -> bool {
        self.open.load(Ordering::Acquire)
    }
}
