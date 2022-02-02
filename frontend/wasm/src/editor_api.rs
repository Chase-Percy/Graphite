//! This file is where functions are defined to be called directly from JS.
//! It serves as a thin wrapper over the editor backend API that relies
//! on the dispatcher messaging system and more complex Rust data types.

use crate::canvas::RenderingContext;
use crate::helpers::{translate_key, Error};
use crate::{EDITOR_HAS_CRASHED, EDITOR_INSTANCES, JS_EDITOR_HANDLES};

use editor::consts::{DEFAULT_FONT_FAMILY, DEFAULT_FONT_STYLE, FILE_SAVE_SUFFIX, GRAPHITE_DOCUMENT_VERSION};
use editor::input::input_preprocessor::ModifierKeys;
use editor::input::mouse::{EditorMouseState, ScrollDelta, ViewportBounds};
use editor::message_prelude::*;
use editor::Color;
use editor::Editor;
use editor::LayerId;
use graphene::Operation;

use serde::Serialize;
use serde_wasm_bindgen::{self, from_value};
use std::sync::atomic::Ordering;
use wasm_bindgen::prelude::*;

/// Set the random seed used by the editor by calling this from JS upon initialization.
/// This is necessary because WASM doesn't have a random number generator.
#[wasm_bindgen]
pub fn set_random_seed(seed: u64) {
	editor::communication::set_uuid_seed(seed);
}

/// Access a handle to WASM memory
#[wasm_bindgen]
pub fn wasm_memory() -> JsValue {
	wasm_bindgen::memory()
}

// To avoid wasm-bindgen from checking mutable reference issues using WasmRefCell we must make all methods take a non mutable reference to self.
// Not doing this creates an issue when rust calls into JS which calls back to rust in the same call stack.
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct JsEditorHandle {
	editor_id: u64,
	frontend_message_handler_callback: js_sys::Function,
	pub(crate) renderer: Option<RenderingContext>,
}

#[wasm_bindgen]
#[allow(clippy::too_many_arguments)]
impl JsEditorHandle {
	#[wasm_bindgen(constructor)]
	pub fn new(frontend_message_handler_callback: js_sys::Function) -> Self {
		let editor_id = generate_uuid();
		let editor = Editor::new();
		let editor_handle = JsEditorHandle {
			editor_id,
			frontend_message_handler_callback,
			renderer: None,
		};
		EDITOR_INSTANCES.with(|instances| instances.borrow_mut().insert(editor_id, editor));
		JS_EDITOR_HANDLES.with(|instances| instances.borrow_mut().insert(editor_id, editor_handle.clone()));
		editor_handle
	}

	// Sends a message to the dispatcher in the Editor Backend
	fn dispatch<T: Into<Message>>(&mut self, message: T) {
		// Process no further messages after a crash to avoid spamming the console
		if EDITOR_HAS_CRASHED.load(Ordering::SeqCst) {
			return;
		}

		let frontend_messages = EDITOR_INSTANCES.with(|instances| {
			instances.try_borrow_mut().map(|mut editors| {
				editors
					.get_mut(&self.editor_id)
					.expect("EDITOR_INSTANCES does not contain the current editor_id")
					.handle_message(message.into())
			})
		});

		if let Ok(frontend_messages) = frontend_messages {
			for message in frontend_messages.into_iter() {
				// Send each FrontendMessage to the JavaScript frontend
				// Send each FrontendMessage to the JavaScript frontend
				match message {
					//FrontendMessage::UpdatePropertyPanelOptionsLayout { .. } => return,
					//FrontendMessage::UpdateToolOptionsLayout { .. } => return,
					//FrontendMessage::UpdateDocumentBarLayout { .. } => return,
					FrontendMessage::UpdateDocumentArtwork { .. } | FrontendMessage::UpdateDocumentOverlays { .. } => {
						JS_EDITOR_HANDLES.with(|instances| {
							instances.borrow_mut().get_mut(&self.editor_id).unwrap().renderer();
						});
					}
					//FrontendMessage::UpdateDocumentScrollbars { .. } => return,
					//FrontendMessage::UpdateDocumentRulers { .. } => return,
					message => {
						//log::debug!("{response:?}");
						self.send_frontend_message_to_js(message);
					}
				}
			}
		}
		// If the editor cannot be borrowed then it has encountered a panic - we should just ignore new dispatches
	}

	// Sends a FrontendMessage to JavaScript
	pub(crate) fn send_frontend_message_to_js(&self, message: FrontendMessage) {
		let message_type = message.to_discriminant().local_name();

		let serializer = serde_wasm_bindgen::Serializer::new().serialize_large_number_types_as_bigints(true);
		let message_data = message.serialize(&serializer).expect("Failed to serialize FrontendMessage");

		let js_return_value = self.frontend_message_handler_callback.call2(&JsValue::null(), &JsValue::from(message_type), &message_data);

		if let Err(error) = js_return_value {
			log::error!(
				"While handling FrontendMessage \"{:?}\", JavaScript threw an error: {:?}",
				message.to_discriminant().local_name(),
				error,
			)
		}
	}

	pub(crate) fn renderer(&mut self) -> &mut RenderingContext {
		if self.renderer.is_none() {
			self.renderer = Some(RenderingContext::new().unwrap());
		}
		self.renderer.as_mut().unwrap()
	}

	// ========================================================================
	// Add additional JS -> Rust wrapper functions below as needed for calling
	// the backend from the web frontend.
	// ========================================================================

	pub fn init_app(&mut self) {
		let message = PortfolioMessage::UpdateOpenDocumentsList;
		self.dispatch(message);

		let message = PortfolioMessage::UpdateDocumentWidgets;
		self.dispatch(message);

		let message = PropertiesPanelMessage::Init;
		self.dispatch(message);

		let message = ToolMessage::InitTools;
		self.dispatch(message);

		// A default font
		let font = graphene::layers::text_layer::Font::new(DEFAULT_FONT_FAMILY.into(), DEFAULT_FONT_STYLE.into());
		let message = FrontendMessage::TriggerFontLoad { font, is_default: true };
		self.dispatch(message);

		let message = MovementMessage::TranslateCanvas { delta: (0., 0.).into() };
		self.dispatch(message);

		let message = MenuBarMessage::SendLayout;
		self.dispatch(message);
	}

	/// Displays a dialog with an error message
	pub fn error_dialog(&mut self, title: String, description: String) {
		let message = DialogMessage::DisplayDialogError { title, description };
		self.dispatch(message);
	}

	/// Answer whether or not the editor has crashed
	pub fn has_crashed(&self) -> bool {
		EDITOR_HAS_CRASHED.load(Ordering::SeqCst)
	}

	/// Get the constant `FILE_SAVE_SUFFIX`
	#[wasm_bindgen]
	pub fn file_save_suffix(&self) -> String {
		FILE_SAVE_SUFFIX.into()
	}

	///
	pub fn draw_overlays(&mut self) {
		EDITOR_INSTANCES.with(|instances| {
			let instances = instances.borrow();
			let editor = instances.get(&self.editor_id).expect("EDITOR_INSTANCES does not contain the current editor_id");
			let lines = editor
				.dispatcher
				.message_handlers
				.portfolio_message_handler
				.active_document()
				.overlays_message_handler
				.overlays_graphene_document
				.root
				.line_iter();
			self.renderer().draw_paths(lines);
		});
	}

	/// Get the constant `GRAPHITE_DOCUMENT_VERSION`
	#[wasm_bindgen]
	pub fn graphite_document_version(&self) -> String {
		GRAPHITE_DOCUMENT_VERSION.to_string()
	}

	/// Update layout of a given UI
	pub fn update_layout(&mut self, layout_target: JsValue, widget_id: u64, value: JsValue) -> Result<(), JsValue> {
		match (from_value(layout_target), from_value(value)) {
			(Ok(layout_target), Ok(value)) => {
				let message = LayoutMessage::UpdateLayout { layout_target, widget_id, value };
				self.dispatch(message);
				Ok(())
			}
			_ => Err(Error::new("Could not update UI").into()),
		}
	}

	pub fn select_document(&mut self, document_id: u64) {
		let message = PortfolioMessage::SelectDocument { document_id };
		self.dispatch(message);
	}

	pub fn open_document_file(&mut self, document_name: String, document_serialized_content: String) {
		let message = PortfolioMessage::OpenDocumentFile {
			document_name,
			document_serialized_content,
		};
		self.dispatch(message);
	}

	pub fn open_auto_saved_document(&mut self, document_id: u64, document_name: String, document_is_saved: bool, document_serialized_content: String) {
		let message = PortfolioMessage::OpenDocumentFileWithId {
			document_id,
			document_name,
			document_is_saved,
			document_serialized_content,
		};
		self.dispatch(message);
	}

	pub fn trigger_auto_save(&mut self, document_id: u64) {
		let message = PortfolioMessage::AutoSaveDocument { document_id };
		self.dispatch(message);
	}

	pub fn close_document_with_confirmation(&mut self, document_id: u64) {
		let message = PortfolioMessage::CloseDocumentWithConfirmation { document_id };
		self.dispatch(message);
	}

	pub fn request_about_graphite_dialog_with_localized_commit_date(&mut self, localized_commit_date: String) {
		let message = DialogMessage::RequestAboutGraphiteDialogWithLocalizedCommitDate { localized_commit_date };
		self.dispatch(message);
	}

	pub fn request_coming_soon_dialog(&mut self, issue: Option<i32>) {
		let message = DialogMessage::RequestComingSoonDialog { issue };
		self.dispatch(message);
	}

	/// Send new bounds when document panel viewports get resized or moved within the editor
	/// [left, top, right, bottom]...
	pub fn bounds_of_viewports(&mut self, bounds_of_viewports: &[f64]) {
		let chunked: Vec<_> = bounds_of_viewports.chunks(4).map(ViewportBounds::from_slice).collect();

		let message = InputPreprocessorMessage::BoundsOfViewports { bounds_of_viewports: chunked };
		self.dispatch(message);
	}

	/// Mouse movement within the screenspace bounds of the viewport
	pub fn on_mouse_move(&mut self, x: f64, y: f64, mouse_keys: u8, modifiers: u8) {
		let editor_mouse_state = EditorMouseState::from_keys_and_editor_position(mouse_keys, (x, y).into());

		let modifier_keys = ModifierKeys::from_bits(modifiers).expect("Invalid modifier keys");

		let message = InputPreprocessorMessage::PointerMove { editor_mouse_state, modifier_keys };
		self.dispatch(message);
	}

	/// Mouse scrolling within the screenspace bounds of the viewport
	pub fn on_mouse_scroll(&mut self, x: f64, y: f64, mouse_keys: u8, wheel_delta_x: i32, wheel_delta_y: i32, wheel_delta_z: i32, modifiers: u8) {
		let mut editor_mouse_state = EditorMouseState::from_keys_and_editor_position(mouse_keys, (x, y).into());
		editor_mouse_state.scroll_delta = ScrollDelta::new(wheel_delta_x, wheel_delta_y, wheel_delta_z);

		let modifier_keys = ModifierKeys::from_bits(modifiers).expect("Invalid modifier keys");

		let message = InputPreprocessorMessage::MouseScroll { editor_mouse_state, modifier_keys };
		self.dispatch(message);
	}

	/// A mouse button depressed within screenspace the bounds of the viewport
	pub fn on_mouse_down(&mut self, x: f64, y: f64, mouse_keys: u8, modifiers: u8) {
		let editor_mouse_state = EditorMouseState::from_keys_and_editor_position(mouse_keys, (x, y).into());

		let modifier_keys = ModifierKeys::from_bits(modifiers).expect("Invalid modifier keys");

		let message = InputPreprocessorMessage::PointerDown { editor_mouse_state, modifier_keys };
		self.dispatch(message);
	}

	/// A mouse button released
	pub fn on_mouse_up(&mut self, x: f64, y: f64, mouse_keys: u8, modifiers: u8) {
		let editor_mouse_state = EditorMouseState::from_keys_and_editor_position(mouse_keys, (x, y).into());

		let modifier_keys = ModifierKeys::from_bits(modifiers).expect("Invalid modifier keys");

		let message = InputPreprocessorMessage::PointerUp { editor_mouse_state, modifier_keys };
		self.dispatch(message);
	}

	/// Mouse double clicked
	pub fn on_double_click(&mut self, x: f64, y: f64, mouse_keys: u8, modifiers: u8) {
		let editor_mouse_state = EditorMouseState::from_keys_and_editor_position(mouse_keys, (x, y).into());
		let modifier_keys = ModifierKeys::from_bits(modifiers).expect("Invalid modifier keys");

		let message = InputPreprocessorMessage::DoubleClick { editor_mouse_state, modifier_keys };
		self.dispatch(message);
	}

	/// A keyboard button depressed within screenspace the bounds of the viewport
	pub fn on_key_down(&mut self, name: String, modifiers: u8) {
		let key = translate_key(&name);
		let modifier_keys = ModifierKeys::from_bits(modifiers).expect("Invalid modifier keys");

		log::trace!("Key down {:?}, name: {}, modifiers: {:?}", key, name, modifiers);

		let message = InputPreprocessorMessage::KeyDown { key, modifier_keys };
		self.dispatch(message);
	}

	/// A keyboard button released
	pub fn on_key_up(&mut self, name: String, modifiers: u8) {
		let key = translate_key(&name);
		let modifier_keys = ModifierKeys::from_bits(modifiers).expect("Invalid modifier keys");

		log::trace!("Key up {:?}, name: {}, modifiers: {:?}", key, name, modifier_keys);

		let message = InputPreprocessorMessage::KeyUp { key, modifier_keys };
		self.dispatch(message);
	}

	/// A text box was committed
	pub fn on_change_text(&mut self, new_text: String) -> Result<(), JsValue> {
		let message = TextMessage::TextChange { new_text };
		self.dispatch(message);

		Ok(())
	}

	/// A font has been downloaded
	pub fn on_font_load(&mut self, font_family: String, font_style: String, preview_url: String, data: Vec<u8>, is_default: bool) -> Result<(), JsValue> {
		let message = PortfolioMessage::FontLoaded {
			font_family,
			font_style,
			preview_url,
			data,
			is_default,
		};
		self.dispatch(message);

		Ok(())
	}

	/// A text box was changed
	pub fn update_bounds(&mut self, new_text: String) -> Result<(), JsValue> {
		let message = TextMessage::UpdateBounds { new_text };
		self.dispatch(message);

		Ok(())
	}

	/// Update primary color
	pub fn update_primary_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32) -> Result<(), JsValue> {
		let primary_color = match Color::from_rgbaf32(red, green, blue, alpha) {
			Some(color) => color,
			None => return Err(Error::new("Invalid color").into()),
		};

		let message = ToolMessage::SelectPrimaryColor { color: primary_color };
		self.dispatch(message);

		Ok(())
	}

	/// Update secondary color
	pub fn update_secondary_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32) -> Result<(), JsValue> {
		let secondary_color = match Color::from_rgbaf32(red, green, blue, alpha) {
			Some(color) => color,
			None => return Err(Error::new("Invalid color").into()),
		};

		let message = ToolMessage::SelectSecondaryColor { color: secondary_color };
		self.dispatch(message);

		Ok(())
	}

	/// Swap primary and secondary color
	pub fn swap_colors(&mut self) {
		let message = ToolMessage::SwapColors;
		self.dispatch(message);
	}

	/// Reset primary and secondary colors to their defaults
	pub fn reset_colors(&mut self) {
		let message = ToolMessage::ResetColors;
		self.dispatch(message);
	}

	/// Paste layers from a serialized json representation
	pub fn paste_serialized_data(&mut self, data: String) {
		let message = PortfolioMessage::PasteSerializedData { data };
		self.dispatch(message);
	}

	/// Modify the layer selection based on the layer which is clicked while holding down the <kbd>Ctrl</kbd> and/or <kbd>Shift</kbd> modifier keys used for range selection behavior
	pub fn select_layer(&mut self, layer_path: Vec<LayerId>, ctrl: bool, shift: bool) {
		let message = DocumentMessage::SelectLayer { layer_path, ctrl, shift };
		self.dispatch(message);
	}

	/// Deselect all layers
	pub fn deselect_all_layers(&mut self) {
		let message = DocumentMessage::DeselectAllLayers;
		self.dispatch(message);
	}

	/// Move a layer to be next to the specified neighbor
	pub fn move_layer_in_tree(&mut self, folder_path: Vec<LayerId>, insert_index: isize) {
		let message = DocumentMessage::MoveSelectedLayersTo {
			folder_path,
			insert_index,
			reverse_index: true,
		};
		self.dispatch(message);
	}

	/// Set the name for the layer
	pub fn set_layer_name(&mut self, layer_path: Vec<LayerId>, name: String) {
		let message = DocumentMessage::SetLayerName { layer_path, name };
		self.dispatch(message);
	}

	/// Translates document (in viewport coords)
	pub fn translate_canvas(&mut self, delta_x: f64, delta_y: f64) {
		let message = MovementMessage::TranslateCanvas { delta: (delta_x, delta_y).into() };
		self.dispatch(message);
	}

	/// Translates document (in viewport coords)
	pub fn translate_canvas_by_fraction(&mut self, delta_x: f64, delta_y: f64) {
		let message = MovementMessage::TranslateCanvasByViewportFraction { delta: (delta_x, delta_y).into() };
		self.dispatch(message);
	}

	/// Sends the blob url generated by js
	pub fn set_image_blob_url(&mut self, path: Vec<LayerId>, blob_url: String, width: f64, height: f64) {
		let dimensions = (width, height);
		let message = Operation::SetImageBlobUrl { path, blob_url, dimensions };
		self.dispatch(message);
	}

	/// Pastes an image
	pub fn paste_image(&mut self, mime: String, image_data: Vec<u8>, mouse_x: Option<f64>, mouse_y: Option<f64>) {
		let mouse = mouse_x.and_then(|x| mouse_y.map(|y| (x, y)));
		let message = DocumentMessage::PasteImage { mime, image_data, mouse };
		self.dispatch(message);
	}

	/// Toggle visibility of a layer from the layer list
	pub fn toggle_layer_visibility(&mut self, layer_path: Vec<LayerId>) {
		let message = DocumentMessage::ToggleLayerVisibility { layer_path };
		self.dispatch(message);
	}

	/// Toggle expansions state of a layer from the layer list
	pub fn toggle_layer_expansion(&mut self, layer_path: Vec<LayerId>) {
		let message = DocumentMessage::ToggleLayerExpansion { layer_path };
		self.dispatch(message);
	}
}

// Needed to make JsEditorHandle functions pub to Rust.
// The reason is not fully clear but it has to do with the #[wasm_bindgen] procedural macro.
impl JsEditorHandle {
	pub fn send_frontend_message_to_js_rust_proxy(&self, message: FrontendMessage) {
		self.send_frontend_message_to_js(message);
	}
}

impl Drop for JsEditorHandle {
	fn drop(&mut self) {
		EDITOR_INSTANCES.with(|instances| instances.borrow_mut().remove(&mut self.editor_id));
	}
}
