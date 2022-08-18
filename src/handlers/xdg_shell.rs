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
		self.output
			.enter(&self.display_handle, surface.wl_surface());
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
		self.output
			.enter(&self.display_handle, surface.wl_surface());
		let _ = surface.send_configure();
		compositor::with_states(surface.wl_surface(), |data| {
			data.data_map
				.insert_if_missing(|| CoreSurface::new(surface.wl_surface().clone()));
			data.data_map
				.insert_if_missing(|| XdgPopup::new(surface.clone(), positioner));
		});
	}

	fn grab(
		&mut self,
		_dh: &smithay::reexports::wayland_server::DisplayHandle,
		_surface: smithay::wayland::shell::xdg::PopupSurface,
		_seat: smithay::reexports::wayland_server::protocol::wl_seat::WlSeat,
		_serial: smithay::wayland::Serial,
	) {
		todo!()
	}
}
delegate_xdg_shell!(Flatland);
