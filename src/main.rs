mod flatland;
mod handlers;
mod surface;

use std::{ffi::c_void, sync::Arc};

use anyhow::{ensure, Result};
use flatland::{ClientState, Flatland};
use sk::lifecycle::Settings;
use slog::Drain;
use smithay::{
	backend::{egl::EGLContext, renderer::gles2::Gles2Renderer},
	reexports::wayland_server::{Display, ListeningSocket},
	wayland::compositor,
};
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

pub static STARDUST_PNG: &[u8] = include_bytes!("../../stardust-assets/icon.png");

fn main() -> Result<()> {
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

	let mut display: Display<Flatland> = Display::new()?;
	let socket = ListeningSocket::bind_auto("wayland", 1..33)?;
	let mut flatland = Flatland::new(&display, renderer, log)?;
	stereokit.run(
		|draw_ctx| {
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
				}
			});
		},
		|| {},
	);

	Ok(())
}

// #[test]
// fn graphics() -> Result<()> {
// 	let log = ::slog::Logger::root(::slog_stdlog::StdLog.fuse(), slog::o!());
// 	slog_stdlog::init()?;

// 	let stereokit = Settings::default()
// 		.app_name("Flatland Wayland Compositor")
// 		.init()
// 		.expect("StereoKit failed to initialize");

// 	let egl_raw_handles = get_sk_egl()?;
// 	let mut renderer = unsafe {
// 		Gles2Renderer::new(
// 			EGLContext::from_raw(
// 				egl_raw_handles.display,
// 				egl_raw_handles.config,
// 				egl_raw_handles.context,
// 				log.clone(),
// 			)?,
// 			log.clone(),
// 		)?
// 	};
// 	let smithay_tex_image =
// 		image::io::Reader::with_format(std::io::Cursor::new(STARDUST_PNG), image::ImageFormat::Png)
// 			.decode()
// 			.unwrap();
// 	let smithay_tex = renderer
// 		.import_memory(
// 			&smithay_tex_image.to_rgba8(),
// 			(
// 				smithay_tex_image.width() as i32,
// 				smithay_tex_image.height() as i32,
// 			)
// 				.into(),
// 			false,
// 		)
// 		.unwrap();
// 	let smithay_sk_tex =
// 		Texture::create(&stereokit, TextureType::Image, TextureFormat::RGBA32Linear)?;
// 	unsafe {
// 		smithay_sk_tex.set_native(
// 			smithay_tex.tex_id() as u32,
// 			smithay::backend::renderer::gles2::ffi::SRGB,
// 			TextureType::Image,
// 			smithay_tex_image.width(),
// 			smithay_tex_image.height(),
// 		);
// 		smithay_sk_tex.set_sample(TextureSample::Linear);
// 	}
// 	let stereokit_sk_tex = Texture::from_mem(&stereokit, STARDUST_PNG, true, 100)?;

// 	let cube_mesh = Mesh::gen_cube(
// 		&stereokit,
// 		mint::Vector3 {
// 			x: 0.1_f32,
// 			y: 0.1_f32,
// 			z: 0.1_f32,
// 		},
// 		1,
// 	)?;
// 	let cube_material_smithay = Material::copy_from_id(&stereokit, DEFAULT_ID_MATERIAL_UNLIT_CLIP)?;
// 	cube_material_smithay.set_parameter("diffuse", &smithay_sk_tex);
// 	let cube_model_smithay = Model::from_mesh(&stereokit, &cube_mesh, &cube_material_smithay)?;

// 	let cube_material_sk = Material::copy_from_id(&stereokit, DEFAULT_ID_MATERIAL_UNLIT_CLIP)?;
// 	cube_material_sk.set_parameter("diffuse", &stereokit_sk_tex);
// 	let cube_model_sk = Model::from_mesh(&stereokit, &cube_mesh, &cube_material_sk)?;
// 	stereokit.run(
// 		|_sk, draw_ctx| {
// 			cube_model_smithay.draw(
// 				draw_ctx,
// 				Mat4::IDENTITY.into(),
// 				Rgba::new(Rgb::new(1_f32, 1_f32, 1_f32), 1_f32).into(),
// 				RenderLayer::Layer0,
// 			);
// 			cube_model_sk.draw(
// 				draw_ctx,
// 				Mat4::from_translation(vec3(0., 0.2, 0.)).into(),
// 				Rgba::new(Rgb::new(1_f32, 1_f32, 1_f32), 1_f32).into(),
// 				RenderLayer::Layer0,
// 			);
// 		},
// 		|| {},
// 	);

// 	Ok(())
// }
