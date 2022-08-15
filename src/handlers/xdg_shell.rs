use crate::{
	flatland::Flatland,
	surface::{CoreSurface, XdgPopup, XdgTopLevel},
};
use smithay::{
	delegate_xdg_shell,
	wayland::{compositor, shell::xdg::XdgShellHandler},
};
use stereokit::pose::Pose;

impl XdgShellHandler for Flatland {
	fn xdg_shell_state(&mut self) -> &mut smithay::wayland::shell::xdg::XdgShellState {
		&mut self.xdg_shell_state
	}

	fn new_toplevel(
		&mut self,
		_dh: &smithay::reexports::wayland_server::DisplayHandle,
		surface: smithay::wayland::shell::xdg::ToplevelSurface,
	) {
		surface.send_configure();
		compositor::with_states(surface.wl_surface(), |data| {
			data.data_map
				.insert_if_missing(|| CoreSurface::new(surface.wl_surface().clone()));
			data.data_map
				.insert_if_missing(|| XdgTopLevel::new(surface.clone(), Pose::IDENTITY));
		});
	}

	fn new_popup(
		&mut self,
		_dh: &smithay::reexports::wayland_server::DisplayHandle,
		surface: smithay::wayland::shell::xdg::PopupSurface,
		positioner: smithay::wayland::shell::xdg::PositionerState,
	) {
		compositor::with_states(surface.wl_surface(), |data| {
			data.data_map
				.insert_if_missing(|| CoreSurface::new(surface.wl_surface().clone()));
			data.data_map.insert_if_missing(|| XdgPopup {
				shell_surf: surface.clone(),
				positioner,
			});
		});
	}

	fn move_request(
		&mut self,
		dh: &smithay::reexports::wayland_server::DisplayHandle,
		surface: smithay::wayland::shell::xdg::ToplevelSurface,
		seat: smithay::reexports::wayland_server::protocol::wl_seat::WlSeat,
		serial: smithay::wayland::Serial,
	) {
	}

	fn resize_request(
		&mut self,
		dh: &smithay::reexports::wayland_server::DisplayHandle,
		surface: smithay::wayland::shell::xdg::ToplevelSurface,
		seat: smithay::reexports::wayland_server::protocol::wl_seat::WlSeat,
		serial: smithay::wayland::Serial,
		edges: smithay::reexports::wayland_protocols::xdg::shell::server::xdg_toplevel::ResizeEdge,
	) {
	}

	fn grab(
		&mut self,
		dh: &smithay::reexports::wayland_server::DisplayHandle,
		surface: smithay::wayland::shell::xdg::PopupSurface,
		seat: smithay::reexports::wayland_server::protocol::wl_seat::WlSeat,
		serial: smithay::wayland::Serial,
	) {
	}

	fn maximize_request(
		&mut self,
		dh: &smithay::reexports::wayland_server::DisplayHandle,
		surface: smithay::wayland::shell::xdg::ToplevelSurface,
	) {
	}

	fn unmaximize_request(
		&mut self,
		dh: &smithay::reexports::wayland_server::DisplayHandle,
		surface: smithay::wayland::shell::xdg::ToplevelSurface,
	) {
	}

	fn fullscreen_request(
		&mut self,
		dh: &smithay::reexports::wayland_server::DisplayHandle,
		surface: smithay::wayland::shell::xdg::ToplevelSurface,
		output: Option<smithay::reexports::wayland_server::protocol::wl_output::WlOutput>,
	) {
	}

	fn unfullscreen_request(
		&mut self,
		dh: &smithay::reexports::wayland_server::DisplayHandle,
		surface: smithay::wayland::shell::xdg::ToplevelSurface,
	) {
	}

	fn minimize_request(
		&mut self,
		dh: &smithay::reexports::wayland_server::DisplayHandle,
		surface: smithay::wayland::shell::xdg::ToplevelSurface,
	) {
	}

	fn show_window_menu(
		&mut self,
		dh: &smithay::reexports::wayland_server::DisplayHandle,
		surface: smithay::wayland::shell::xdg::ToplevelSurface,
		seat: smithay::reexports::wayland_server::protocol::wl_seat::WlSeat,
		serial: smithay::wayland::Serial,
		location: smithay::utils::Point<i32, smithay::utils::Logical>,
	) {
	}

	fn ack_configure(
		&mut self,
		dh: &smithay::reexports::wayland_server::DisplayHandle,
		surface: smithay::reexports::wayland_server::protocol::wl_surface::WlSurface,
		configure: smithay::wayland::shell::xdg::Configure,
	) {
	}

	fn reposition_request(
		&mut self,
		dh: &smithay::reexports::wayland_server::DisplayHandle,
		surface: smithay::wayland::shell::xdg::PopupSurface,
		positioner: smithay::wayland::shell::xdg::PositionerState,
		token: u32,
	) {
	}
}
delegate_xdg_shell!(Flatland);
