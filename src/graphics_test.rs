use crate::get_sk_egl;
use anyhow::Result;
use glam::{vec3, Mat4};
use prisma::{Rgb, Rgba};
use slog::Drain;
use smithay::backend::{
	egl::EGLContext,
	renderer::{gles2::Gles2Renderer, ImportMem},
};
use stereokit::{
	enums::RenderLayer,
	material::{Material, DEFAULT_ID_MATERIAL_UNLIT_CLIP},
	mesh::Mesh,
	model::Model,
	texture::{Texture, TextureFormat, TextureSample, TextureType},
	Settings,
};

pub static STARDUST_PNG: &[u8] = include_bytes!("../../stardust-assets/icon.png");

#[test]
fn grahics() -> Result<()> {
	let log = ::slog::Logger::root(::slog_stdlog::StdLog.fuse(), slog::o!());
	slog_stdlog::init().expect("Logger failed to initialize");

	let stereokit = Settings::default()
		.app_name("Flatland Wayland Compositor")
		.init()
		.unwrap();

	let egl_raw_handles = get_sk_egl()?;
	let mut renderer = unsafe {
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
	let smithay_tex_image =
		image::io::Reader::with_format(std::io::Cursor::new(STARDUST_PNG), image::ImageFormat::Png)
			.decode()
			.unwrap();
	let smithay_tex = renderer
		.import_memory(
			&smithay_tex_image.to_rgba8(),
			(
				smithay_tex_image.width() as i32,
				smithay_tex_image.height() as i32,
			)
				.into(),
			false,
		)
		.unwrap();
	let smithay_sk_tex =
		Texture::create(&stereokit, TextureType::Image, TextureFormat::RGBA32).unwrap();
	unsafe {
		smithay_sk_tex.set_native(
			smithay_tex.tex_id() as usize,
			smithay::backend::renderer::gles2::ffi::SRGB.into(),
			TextureType::Image,
			smithay_tex_image.width(),
			smithay_tex_image.height(),
		);
		smithay_sk_tex.set_sample(TextureSample::Linear);
	}
	let stereokit_sk_tex = Texture::from_mem(&stereokit, STARDUST_PNG, true, 100).unwrap();

	let cube_mesh = Mesh::gen_cube(
		&stereokit,
		mint::Vector3 {
			x: 0.1_f32,
			y: 0.1_f32,
			z: 0.1_f32,
		},
		1,
	)
	.unwrap();
	let cube_material_smithay =
		Material::copy_from_id(&stereokit, DEFAULT_ID_MATERIAL_UNLIT_CLIP).unwrap();
	cube_material_smithay.set_parameter("diffuse", &smithay_sk_tex);
	let cube_model_smithay =
		Model::from_mesh(&stereokit, &cube_mesh, &cube_material_smithay).unwrap();

	let cube_material_sk =
		Material::copy_from_id(&stereokit, DEFAULT_ID_MATERIAL_UNLIT_CLIP).unwrap();
	cube_material_sk.set_parameter("diffuse", &stereokit_sk_tex);
	let cube_model_sk = Model::from_mesh(&stereokit, &cube_mesh, &cube_material_sk).unwrap();
	stereokit.run(
		|draw_ctx| {
			cube_model_smithay.draw(
				draw_ctx,
				Mat4::IDENTITY.into(),
				Rgba::new(Rgb::new(1_f32, 1_f32, 1_f32), 1_f32).into(),
				RenderLayer::Layer0,
			);
			cube_model_sk.draw(
				draw_ctx,
				Mat4::from_translation(vec3(0., 0.2, 0.)).into(),
				Rgba::new(Rgb::new(1_f32, 1_f32, 1_f32), 1_f32).into(),
				RenderLayer::Layer0,
			);
		},
		|| {},
	);

	Ok(())
}
