use anyhow::Result;
use slog::Logger;
use smithay::{
	backend::renderer::gles2::Gles2Renderer,
	reexports::wayland_server::{
		backend::{ClientData, ClientId, DisconnectReason},
		Display,
	},
	wayland::{
		compositor::CompositorState, output::OutputManagerState, shell::xdg::XdgShellState,
		shm::ShmState,
	},
};

pub struct ClientState;
impl ClientData for ClientState {
	fn initialized(&self, _client_id: ClientId) {
		println!("initialized");
	}

	fn disconnected(&self, _client_id: ClientId, _reason: DisconnectReason) {
		println!("disconnected");
	}
}

pub struct Flatland {
	pub log: slog::Logger,

	pub renderer: Gles2Renderer,
	pub compositor_state: CompositorState,
	pub xdg_shell_state: XdgShellState,
	pub shm_state: ShmState,
	// pub output_manager_state: OutputManagerState,
	// pub seat_state: SeatState<Flatland>,
	// pub data_device_state: DataDeviceState,
}

impl Flatland {
	pub fn new(display: &Display<Flatland>, renderer: Gles2Renderer, log: Logger) -> Result<Self> {
		let dh = display.handle();

		let compositor_state = CompositorState::new::<Self, _>(&dh, log.clone());
		let xdg_shell_state = XdgShellState::new::<Self, _>(&dh, log.clone());
		let shm_state = ShmState::new::<Self, _>(&dh, vec![], log.clone());
		// let output_manager_state = OutputManagerState::new_with_xdg_output(&dh);
		// let seat_state = SeatState::new();
		// let data_device_state = DataDeviceState::new(&dh, log.clone());

		Ok(Flatland {
			log,
			renderer,
			compositor_state,
			xdg_shell_state,
			shm_state,
			// output_manager_state,
			// seat_state,
			// data_device_state,
		})
	}
}
