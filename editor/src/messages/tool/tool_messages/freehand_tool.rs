use crate::messages::frontend::utility_types::MouseCursorIcon;
use crate::messages::input_mapper::utility_types::input_keyboard::MouseMotion;
use crate::messages::layout::utility_types::layout_widget::{Layout, LayoutGroup, PropertyHolder, WidgetLayout};
use crate::messages::layout::utility_types::widgets::input_widgets::NumberInput;
use crate::messages::prelude::*;
use crate::messages::tool::common_functionality::graph_modification_utils;
use crate::messages::tool::utility_types::{DocumentToolData, EventToMessageMap, Fsm, ToolActionHandlerData, ToolMetadata, ToolTransition, ToolType};
use crate::messages::tool::utility_types::{HintData, HintGroup, HintInfo};

use document_legacy::LayerId;
use document_legacy::Operation;
use graphene_core::vector::style::Stroke;

use glam::DVec2;
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct FreehandTool {
	fsm_state: FreehandToolFsmState,
	data: FreehandToolData,
	options: FreehandOptions,
}

pub struct FreehandOptions {
	line_weight: f64,
}

impl Default for FreehandOptions {
	fn default() -> Self {
		Self { line_weight: 5. }
	}
}

#[remain::sorted]
#[impl_message(Message, ToolMessage, Freehand)]
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, specta::Type)]
pub enum FreehandToolMessage {
	// Standard messages
	#[remain::unsorted]
	Abort,

	// Tool-specific messages
	DragStart,
	DragStop,
	PointerMove,
	UpdateOptions(FreehandToolMessageOptionsUpdate),
}

#[remain::sorted]
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, specta::Type)]
pub enum FreehandToolMessageOptionsUpdate {
	LineWeight(f64),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum FreehandToolFsmState {
	#[default]
	Ready,
	Drawing,
}

impl ToolMetadata for FreehandTool {
	fn icon_name(&self) -> String {
		"VectorFreehandTool".into()
	}
	fn tooltip(&self) -> String {
		"Freehand Tool".into()
	}
	fn tool_type(&self) -> crate::messages::tool::utility_types::ToolType {
		ToolType::Freehand
	}
}

impl PropertyHolder for FreehandTool {
	fn properties(&self) -> Layout {
		let weight = NumberInput::new(Some(self.options.line_weight))
			.unit(" px")
			.label("Weight")
			.min(1.)
			.on_update(|number_input: &NumberInput| FreehandToolMessage::UpdateOptions(FreehandToolMessageOptionsUpdate::LineWeight(number_input.value.unwrap())).into())
			.widget_holder();
		Layout::WidgetLayout(WidgetLayout::new(vec![LayoutGroup::Row { widgets: vec![weight] }]))
	}
}

impl<'a> MessageHandler<ToolMessage, &mut ToolActionHandlerData<'a>> for FreehandTool {
	fn process_message(&mut self, message: ToolMessage, responses: &mut VecDeque<Message>, tool_data: &mut ToolActionHandlerData<'a>) {
		if let ToolMessage::Freehand(FreehandToolMessage::UpdateOptions(action)) = message {
			match action {
				FreehandToolMessageOptionsUpdate::LineWeight(line_weight) => self.options.line_weight = line_weight,
			}
			return;
		}

		self.fsm_state.process_event(message, &mut self.data, tool_data, &self.options, responses, true);
	}

	fn actions(&self) -> ActionList {
		use FreehandToolFsmState::*;

		match self.fsm_state {
			Ready => actions!(FreehandToolMessageDiscriminant;
				DragStart,
				DragStop,
				Abort,
			),
			Drawing => actions!(FreehandToolMessageDiscriminant;
				DragStop,
				PointerMove,
				Abort,
			),
		}
	}
}

impl ToolTransition for FreehandTool {
	fn event_to_message_map(&self) -> EventToMessageMap {
		EventToMessageMap {
			document_dirty: None,
			tool_abort: Some(FreehandToolMessage::Abort.into()),
			selection_changed: None,
		}
	}
}

#[derive(Clone, Debug, Default)]
struct FreehandToolData {
	points: Vec<DVec2>,
	weight: f64,
	path: Option<Vec<LayerId>>,
}

impl Fsm for FreehandToolFsmState {
	type ToolData = FreehandToolData;
	type ToolOptions = FreehandOptions;

	fn transition(
		self,
		event: ToolMessage,
		tool_data: &mut Self::ToolData,
		ToolActionHandlerData {
			document, global_tool_data, input, ..
		}: &mut ToolActionHandlerData,
		tool_options: &Self::ToolOptions,
		responses: &mut VecDeque<Message>,
	) -> Self {
		use FreehandToolFsmState::*;
		use FreehandToolMessage::*;

		let transform = document.document_legacy.root.transform;

		if let ToolMessage::Freehand(event) = event {
			match (self, event) {
				(Ready, DragStart) => {
					responses.add(DocumentMessage::StartTransaction);
					responses.add(DocumentMessage::DeselectAllLayers);
					tool_data.path = Some(document.get_path_for_new_layer());

					let pos = transform.inverse().transform_point2(input.mouse.position);

					tool_data.points.push(pos);

					tool_data.weight = tool_options.line_weight;

					add_polyline(tool_data, global_tool_data, responses);

					Drawing
				}
				(Drawing, PointerMove) => {
					let pos = transform.inverse().transform_point2(input.mouse.position);

					if tool_data.points.last() != Some(&pos) {
						tool_data.points.push(pos);
					}

					add_polyline(tool_data, global_tool_data, responses);

					Drawing
				}
				(Drawing, DragStop) | (Drawing, Abort) => {
					if tool_data.points.len() >= 2 {
						responses.add(remove_preview(tool_data));
						add_polyline(tool_data, global_tool_data, responses);
						responses.add(DocumentMessage::CommitTransaction);
					} else {
						responses.add(DocumentMessage::AbortTransaction);
					}

					tool_data.path = None;
					tool_data.points.clear();

					Ready
				}
				_ => self,
			}
		} else {
			self
		}
	}

	fn update_hints(&self, responses: &mut VecDeque<Message>) {
		let hint_data = match self {
			FreehandToolFsmState::Ready => HintData(vec![HintGroup(vec![HintInfo::mouse(MouseMotion::LmbDrag, "Draw Polyline")])]),
			FreehandToolFsmState::Drawing => HintData(vec![]),
		};

		responses.add(FrontendMessage::UpdateInputHints { hint_data });
	}

	fn update_cursor(&self, responses: &mut VecDeque<Message>) {
		responses.add(FrontendMessage::UpdateMouseCursor { cursor: MouseCursorIcon::Default });
	}
}

fn remove_preview(data: &FreehandToolData) -> Message {
	Operation::DeleteLayer { path: data.path.clone().unwrap() }.into()
}

fn add_polyline(data: &FreehandToolData, tool_data: &DocumentToolData, responses: &mut VecDeque<Message>) {
	let subpath = bezier_rs::Subpath::from_anchors(data.points.iter().copied(), false);

	let layer_path = data.path.clone().unwrap();
	graph_modification_utils::new_vector_layer(vec![subpath], layer_path.clone(), responses);

	responses.add(GraphOperationMessage::StrokeSet {
		layer: layer_path,
		stroke: Stroke::new(tool_data.primary_color, data.weight),
	});
}
