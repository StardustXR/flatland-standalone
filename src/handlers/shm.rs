use smithay::{delegate_shm, wayland::shm::ShmHandler};

use crate::flatland::Flatland;

impl ShmHandler for Flatland {
	fn shm_state(&self) -> &smithay::wayland::shm::ShmState {
		&self.shm_state
	}
}
delegate_shm!(Flatland);
