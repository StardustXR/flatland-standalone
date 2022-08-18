mod flatland;
mod handlers;
mod input_window;
mod surface;

#[cfg(test)]
mod graphics_test;

use anyhow::{ensure, Result};
use flatland::{ClientState, Flatland};
use input_window::InputWindow;
use sk::lifecycle::Settings;
use slog::Drain;
use smithay::{
	backend::{egl::EGLContext, renderer::gles2::Gles2Renderer},
	desktop::utils::send_frames_surface_tree,
	reexports::wayland_server::{Display, ListeningSocket},
	wayland::compositor,
};
use std::{ffi::c_void, sync::Arc};
use stereokit as sk;
use surface::{Surface, XdgTopLevel};

struct EGLRawHandles {
	display: *const c_void,
	config: *const c_void,
	context: *const c_void,
}
fn get_sk_egl() -> Result<EGLRawHandles> {
	ensure!(
		unsafe { sk::sys::backend_graphics_get() }
			== sk::sys::backend_graphics__backend_graphics_opengles_egl,
		"StereoKit is not running using EGL!"
	);

	Ok(unsafe {
		EGLRawHandles {
			display: sk::sys::backend_opengl_egl_get_display() as *const c_void,
			config: sk::sys::backend_opengl_egl_get_config() as *const c_void,
			context: sk::sys::backend_opengl_egl_get_context() as *const c_void,
		}
	})
}

fn main() {
	winit_main::run(|event_loop, events| {
		(|| -> Result<()> {
			let log = ::slog::Logger::root(::slog_stdlog::StdLog.fuse(), slog::o!());
			slog_stdlog::init()?;

			let stereokit = Settings::default()
				.app_name("Flatland Wayland Compositor")
				.init()
				.expect("StereoKit failed to initialize");

			let egl_raw_handles = get_sk_egl()?;
			let renderer = unsafe {
				Gles2Renderer::new(
					EGLContext::from_raw(
						egl_raw_handles.display,
						egl_raw_handles.config,
						egl_raw_handles.context,
						log.clone(),
					)?,
					log.clone(),
				)?
			};

			let mut input_window = InputWindow::new(event_loop, events)?;

			let mut display: Display<Flatland> = Display::new()?;
			let socket = ListeningSocket::bind_auto("wayland", 1..33)?;
			let mut flatland = Flatland::new(&display, renderer, log)?;
			stereokit.run(
				|draw_ctx| {
					input_window.handle_events(&stereokit);

					if let Ok(Some(client)) = socket.accept() {
						let _ = display
							.handle()
							.insert_client(client, Arc::new(ClientState));
					}
					display.flush_clients().unwrap();
					display.dispatch_clients(&mut flatland).unwrap();

					flatland.xdg_shell_state.toplevel_surfaces(|surfs| {
						for surf in surfs.iter() {
							compositor::with_states(surf.wl_surface(), |data| {
								let top_level = data.data_map.get::<XdgTopLevel>().unwrap();
								top_level.step(&stereokit, draw_ctx, &data.data_map);
							});
							send_frames_surface_tree(
								surf.wl_surface(),
								(stereokit.time_getf() * 1000.) as u32,
							);
						}
					});
				},
				|| {},
			);
			Ok(())
		})()
		.unwrap();
	});
}
