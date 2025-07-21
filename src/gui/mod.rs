pub mod editor;
pub mod state;
use std::sync::Arc;

use crate::gui::state::ViziaState;

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (400, 150))
}
