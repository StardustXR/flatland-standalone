use smithay::wayland::buffer::BufferHandler;

use crate::flatland::Flatland;

impl BufferHandler for Flatland {
	fn buffer_destroyed(
		&mut self,
		_buffer: &smithay::reexports::wayland_server::protocol::wl_buffer::WlBuffer,
	) {
	}
}
