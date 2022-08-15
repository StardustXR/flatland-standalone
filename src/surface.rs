#![allow(dead_code)]

use glam::{vec3, Mat4, Quat};
use once_cell::unsync::OnceCell;
use prisma::{Rgb, Rgba};
use sk::{
	enums::RenderLayer,
	lifecycle::DrawContext,
	material::{Material, DEFAULT_ID_MATERIAL_UNLIT},
	model::Model,
	pose::Pose,
	shader::Shader,
	texture::{Texture, TextureFormat, TextureSample, TextureType},
	ui::{MoveType, WindowType},
	StereoKit,
};
use smithay::{
	backend::renderer::{gles2::Gles2Texture, utils::RendererSurfaceStateUserData},
	desktop::utils::send_frames_surface_tree,
	reexports::wayland_server::protocol::wl_surface::WlSurface,
	utils::user_data::UserDataMap,
	wayland::shell::xdg::{
		PopupSurface, PositionerState, ToplevelSurface, XdgToplevelSurfaceRoleAttributes,
	},
};
use std::{
	cell::{Cell, RefCell},
	fmt::Error,
	sync::Mutex,
};
use stereokit as sk;

const GAMMA_SHADER_BYTES: &[u8] = include_bytes!("../res/shader_unlit_gamma.sks");
const SIMULA_SHADER_BYTES: &[u8] = include_bytes!("../res/shader_unlit_simula.sks");
const PANEL_MODEL_BYTES: &[u8] = include_bytes!("../res/panel.glb");

pub struct CoreSurface {
	wl_surface: WlSurface,
	pub(crate) wl_tex: RefCell<Option<Gles2Texture>>,
	sk_tex: OnceCell<Texture>,
	sk_model: OnceCell<Model>,
}

impl CoreSurface {
	pub fn new(wl_surface: WlSurface) -> Self {
		CoreSurface {
			wl_surface,
			wl_tex: RefCell::new(None),
			sk_tex: OnceCell::new(),
			sk_model: OnceCell::new(),
		}
	}

	pub fn update_tex(&self, sk: &StereoKit, data: &UserDataMap) {
		self.sk_tex
			.get_or_try_init(|| {
				Texture::create(sk, TextureType::ImageNoMips, TextureFormat::RGBA32).ok_or(Error)
			})
			.unwrap();
		self.sk_model
			.get_or_try_init::<_, Error>(|| {
				// let shader = Shader::from_mem(sk, SIMULA_SHADER_BYTES).unwrap();
				// let material = Material::create(sk, &shader).unwrap();
				let material = Material::copy_from_id(sk, DEFAULT_ID_MATERIAL_UNLIT).unwrap();
				material.set_parameter("diffuse", self.sk_tex.get().unwrap());
				let model = Model::from_mem(sk, "panel.glb", PANEL_MODEL_BYTES, None).unwrap();
				model.set_material(0, &material);
				Ok(model)
			})
			.unwrap();
		if let Some(smithay_tex) = self.wl_tex.borrow().as_ref() {
			let size = data
				.get::<RendererSurfaceStateUserData>()
				.and_then(|f| f.borrow().buffer_size())
				.map(|f| (f.w, f.h))
				.unwrap_or((512, 512));
			let sk_tex = self.sk_tex.get().unwrap();
			unsafe {
				sk_tex.set_native(
					smithay_tex.tex_id() as usize,
					smithay::backend::renderer::gles2::ffi::SRGB.into(),
					TextureType::Image,
					size.0 as u32,
					size.1 as u32,
				);
				sk_tex.set_sample(TextureSample::Point);
			}
		}
	}

	fn draw(&self, _sk: &StereoKit, draw_ctx: &DrawContext, size: (f32, f32)) {
		let model = self.sk_model.get().unwrap();
		model.draw(
			draw_ctx,
			Mat4::from_scale_rotation_translation(
				vec3(size.0, size.1, 0.01),
				Quat::IDENTITY,
				vec3(0., -size.1 / 2., 0.005),
			)
			.into(),
			Rgba::new(Rgb::new(1_f32, 1_f32, 1_f32), 1_f32),
			RenderLayer::Layer0,
		);
	}
}

pub trait Surface {
	fn wl_surface(&self) -> &WlSurface;

	fn step(&self, sk: &StereoKit, draw_ctx: &DrawContext, data_map: &UserDataMap) {
		let attributes = data_map
			.get::<Mutex<XdgToplevelSurfaceRoleAttributes>>()
			.unwrap()
			.lock()
			.unwrap();
		let surf_state = data_map.get::<RendererSurfaceStateUserData>();
		let win_name = attributes.title.as_deref().unwrap_or("Unknown");
		let size = surf_state
			.and_then(|f| f.borrow().surface_size())
			.map(|f| (f.w, f.h))
			.unwrap_or((512, 512));
		self.draw(sk, draw_ctx, data_map, win_name, size);
	}

	fn draw(
		&self,
		sk: &StereoKit,
		draw_ctx: &DrawContext,
		data_map: &UserDataMap,
		name: &str,
		size: (i32, i32),
	);
}

pub struct XdgTopLevel {
	pub(crate) shell_surf: ToplevelSurface,
	pub(crate) pose: RefCell<Pose>,
	pub(crate) ppm: Cell<f32>,
}
impl XdgTopLevel {
	pub fn new(shell_surf: ToplevelSurface, pose: Pose) -> Self {
		XdgTopLevel {
			shell_surf: shell_surf,
			pose: RefCell::new(pose),
			ppm: Cell::new(2000.),
		}
	}
}
impl Surface for XdgTopLevel {
	fn wl_surface(&self) -> &WlSurface {
		self.shell_surf.wl_surface()
	}

	fn draw(
		&self,
		sk: &StereoKit,
		draw_ctx: &DrawContext,
		data_map: &UserDataMap,
		name: &str,
		size: (i32, i32),
	) {
		let ppm = self.ppm.get();
		let size = (size.0 as f32 / ppm, size.1 as f32 / ppm);
		sk::ui::window(
			draw_ctx,
			name,
			&mut self.pose.borrow_mut(),
			mint::Vector2 {
				x: size.0,
				y: size.1,
			},
			WindowType::WindowHead,
			MoveType::MoveFaceUser,
			|_ui| {
				let surf = data_map.get::<CoreSurface>().unwrap();
				surf.update_tex(sk, data_map);
				surf.draw(sk, draw_ctx, size);
			},
		);
	}
}

pub struct XdgPopup {
	pub(crate) shell_surf: PopupSurface,
	pub(crate) positioner: PositionerState,
}
