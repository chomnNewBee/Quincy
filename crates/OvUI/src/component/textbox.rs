use std::fmt::Debug;

use egui::{text_edit::TextEditState, Color32, Stroke, TextBuffer};
use serde::{Deserialize, Serialize};
use OvMacros::Control;
use OvTools::{message::messageSender::MessageSender, utils::r#ref::Ref};
use OvWindowing::{dpi::LogicalPosition, Window};

use crate::message::{ime::ImeMessage, UiMessage, UiMessageType};

use super::{Component, UiNodeTrait};

#[derive(Control, Serialize, Deserialize)]
pub struct TextBox {
    text: String,
    id: Index,
    focus: bool,
    width: f32,
    height: f32,
    state: bool,
}

impl Debug for TextBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TextBox").field("text", &self.text).finish()
    }
}

#[typetag::serde]
impl UiNodeTrait for TextBox {
    fn render(&mut self, ui: &mut egui::Ui, sender: &MessageSender<UiMessage>) {
        let frame = egui::Frame::none();

        frame.show(ui, |ui| {
            ui.scope(|ui| {
                ui.style_mut().visuals.selection.bg_fill = Color32::from_rgb(51, 103, 209);
                ui.style_mut().visuals.widgets.inactive.bg_stroke =
                    Stroke::new(0.5, Color32::BLACK);

                let input = egui::TextEdit::singleline(&mut self.text).text_color(Color32::BLACK);

                let result = ui.add_sized([self.width, self.height], input);

                self.state = true;
            })
        });
    }
}

impl TextBox {
    pub fn new(text: &str) -> TextBox {
        Self {
            text: text.to_string(),
            id: Index::DANGLING,
            focus: false,
            width: 200.,
            height: 30.,
            state: false,
        }
    }
}
