use anyhow::Result;
use std::rc::Rc;
use stereokit::StereoKit;
use winit_main::{
	reexports::{
		dpi::{LogicalPosition, PhysicalPosition},
		event::{
			ElementState, Event, KeyboardInput, ModifiersState, MouseButton, VirtualKeyCode,
			WindowEvent,
		},
		window::{Window, WindowAttributes},
	},
	EventLoopHandle, EventReceiver,
};

pub struct InputWindow {
	#[allow(dead_code)]
	event_loop: EventLoopHandle,
	events: Rc<EventReceiver>,
	window: Window,
	grabbed: bool,
	modifiers: ModifiersState,
}
impl InputWindow {
	pub fn new(
		event_loop: winit_main::EventLoopHandle,
		events: winit_main::EventReceiver,
	) -> Result<Self> {
		let window = event_loop.create_window(WindowAttributes::default())?;
		let mut input_window = InputWindow {
			event_loop,
			events: Rc::new(events),
			window,
			grabbed: true,
			modifiers: ModifiersState::empty(),
		};
		input_window.set_grab(false);

		Ok(input_window)
	}

	pub fn handle_events(&mut self, sk: &StereoKit) {
		let events = self.events.clone();
		for event in events.try_iter() {
			match event {
				Event::WindowEvent { event, .. } => self.handle_window_event(sk, event),
				_ => (),
			}
		}
	}

	fn handle_window_event(&mut self, sk: &StereoKit, event: WindowEvent) {
		match event {
			WindowEvent::CloseRequested => sk.quit(),
			WindowEvent::Destroyed => sk.quit(),
			WindowEvent::KeyboardInput { input, .. } => self.handle_keyboard_input(input),
			WindowEvent::MouseInput { state, button, .. } => self.handle_mouse_input(state, button),
			WindowEvent::ModifiersChanged(state) => self.modifiers = state,
			WindowEvent::CursorMoved { position, .. } => self.handle_mouse_move(position),
			_ => (),
		}
	}

	fn handle_mouse_move(&mut self, position: PhysicalPosition<f64>) {
		let _position = position.to_logical::<u32>(self.window.scale_factor());
		let window_size = self
			.window
			.inner_size()
			.to_logical::<u32>(self.window.scale_factor());

		if self.grabbed {
			let _ = self.window.set_cursor_position(LogicalPosition::new(
				window_size.width / 2,
				window_size.height / 2,
			));
		}
	}

	fn handle_mouse_input(
		&mut self,
		state: ElementState,
		button: MouseButton,
		// modifiers: ModifiersState,
	) {
		if state == ElementState::Released && button == MouseButton::Left {
			self.set_grab(true);
		}
	}

	fn handle_keyboard_input(&mut self, input: KeyboardInput) {
		if input.virtual_keycode == Some(VirtualKeyCode::Escape)
			&& input.state == ElementState::Released
			&& self.modifiers.ctrl()
		{
			self.set_grab(false);
		}
	}

	const GRABBED_WINDOW_TITLE: &'static str = "Flatland Input (ctrl+esc to release cursor)";
	const UNGRABBED_WINDOW_TITLE: &'static str = "Flatland Input (click to grab input)";
	fn set_grab(&mut self, grab: bool) {
		if grab != self.grabbed {
			self.grabbed = grab;

			if self.window.set_cursor_grab(grab).is_ok() {
				self.window.set_title(if grab {
					Self::GRABBED_WINDOW_TITLE
				} else {
					Self::UNGRABBED_WINDOW_TITLE
				});
			}
		}
	}
}
