pub mod xdg_decoration;
pub mod buffer;
pub mod compositor;
pub mod shm;
pub mod xdg_shell;

use crate::flatland::Flatland;
use smithay::delegate_output;
delegate_output!(Flatland);
