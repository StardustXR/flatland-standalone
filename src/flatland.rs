use anyhow::Result;
use slog::Logger;
use smithay::{
	backend::renderer::gles2::Gles2Renderer,
	delegate_output,
	reexports::wayland_server::{
		backend::{ClientData, ClientId, DisconnectReason},
		protocol::wl_output::Subpixel,
		Display, DisplayHandle,
	},
	utils::Size,
	wayland::{
		compositor::CompositorState,
		output::{Output, OutputManagerState, Scale::Integer},
		shell::xdg::XdgShellState,
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

	pub display_handle: DisplayHandle,
	pub renderer: Gles2Renderer,
	pub compositor_state: CompositorState,
	pub xdg_shell_state: XdgShellState,
	pub shm_state: ShmState,
	pub output_manager_state: OutputManagerState,
	pub output: Output,
	// pub seat_state: SeatState<Flatland>,
	// pub data_device_state: DataDeviceState,
}

impl Flatland {
	pub fn new(display: &Display<Flatland>, renderer: Gles2Renderer, log: Logger) -> Result<Self> {
		let display_handle = display.handle();

		let compositor_state = CompositorState::new::<Self, _>(&display_handle, log.clone());
		let xdg_shell_state = XdgShellState::new::<Self, _>(&display_handle, log.clone());
		let shm_state = ShmState::new::<Self, _>(&display_handle, vec![], log.clone());
		let output_manager_state = OutputManagerState::new_with_xdg_output::<Self>(&display_handle);
		let output = Output::new(
			"1x".to_owned(),
			smithay::wayland::output::PhysicalProperties {
				size: Size::default(),
				subpixel: Subpixel::None,
				make: "Virtual XR Display".to_owned(),
				model: "Your Headset Name Here".to_owned(),
			},
			log.clone(),
		);
		let _global = output.create_global::<Self>(&display_handle);
		output.change_current_state(None, None, Some(Integer(2)), None);
		// let seat_state = SeatState::new();
		// let data_device_state = DataDeviceState::new(&dh, log.clone());

		Ok(Flatland {
			log,
			display_handle,
			renderer,
			compositor_state,
			xdg_shell_state,
			shm_state,
			output_manager_state,
			output,
			// seat_state,
			// data_device_state,
		})
	}
}
