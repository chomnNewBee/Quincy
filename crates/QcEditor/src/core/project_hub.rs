use std::sync::mpsc::Sender;

use eframe::EventLoopBuilder;
use egui::{Color32, Vec2};

use QcUI::component::{Button, ButtonMessage, Canvas, Grid, Panel, PanelWindow, ToUi, Widget};
use QcWindowing::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::{run_on_demand::EventLoopExtRunOnDemand, windows::EventLoopBuilderExtWindows},
    settings::WindowSettings,
};

use crate::core::message::Page;

use super::{
    editor::{self, Editor},
    message::EditorMessage,
};

pub struct ProjectHub {
    editor: Editor,
}

impl ProjectHub {
    pub fn new(editor: Editor) -> Self {
        ProjectHub { editor }
        // ProjectHub {}
    }

    pub fn run(mut self) {
        // self.el
        //     .run_on_demand(move |event, el| {
        //         el.set_control_flow(ControlFlow::Poll);

        //         match event {
        //             Event::WindowEvent { window_id, event } => {
        //                 self.editor.pre_update(&event);

        //                 match event {
        //                     WindowEvent::CloseRequested => {
        //                         el.exit();
        //                     }
        //                     WindowEvent::Resized(size) => {
        //                         // let renderer = self.context.renderer.try_read().unwrap();
        //                         // renderer.set_viewport(0, 0, size.width as _, size.height as _);
        //                     }
        //                     _ => {
        //                         // println!("event:{:?}", event);
        //                     }
        //                 }
        //             }
        //             Event::AboutToWait => {
        //                 self.editor.update();
        //                 self.editor.post_update();
        //                 // clock.update();
        //             }

        //             _ => {}
        //         }
        //     })
        //     .unwrap();
    }
}

pub struct TestPanel {
    canvas: Canvas,
    sender: Sender<EditorMessage>,
}

impl TestPanel {
    pub fn new(sender: Sender<EditorMessage>) -> Self {
        let mut hub = TestPanel {
            canvas: Canvas::new(),
            sender,
        };
        hub.init_view();
        hub
    }

    pub fn init_view(&mut self) {
        let sender = self.sender.clone();

        Grid::new(
            Widget::default()
                .with_padding(100f32.into())
                .with_background(Color32::from_rgb(23, 23, 26)),
        )
        .with_columns(3)
        .with_spacing(Vec2::new(20., 20.))
        .with_children(vec![
            Button::new(
                Widget::default()
                    .with_background(Color32::from_rgb(179, 128, 0))
                    .with_height(30.)
                    .with_width(100.)
                    .on_event(
                        ButtonMessage::Clicked.into(),
                        Box::new(move |msg| {
                            println!("切换界面");

                            sender.send(EditorMessage::GoTo(Page::ProjectHub)).unwrap();
                        }),
                    )
                    .build(&mut self.canvas),
            )
            .with_text("切换回去")
            .toUi(),
            Button::new(
                Widget::default()
                    .with_background(Color32::from_rgb(179, 128, 0))
                    .with_height(30.)
                    .with_width(100.),
            )
            .with_text("切换回去")
            .toUi(),
            Button::new(
                Widget::default()
                    .with_background(Color32::from_rgb(179, 128, 0))
                    .with_height(30.)
                    .with_width(100.),
            )
            .with_text("切换回去")
            .toUi(),
            Button::new(
                Widget::default()
                    .with_background(Color32::from_rgb(179, 128, 0))
                    .with_height(30.)
                    .with_width(100.),
            )
            .with_text("切换回去")
            .toUi(),
            Button::new(
                Widget::default()
                    .with_background(Color32::from_rgb(179, 128, 0))
                    .with_height(30.)
                    .with_width(100.),
            )
            .with_text("切换回去")
            .toUi(),
            Button::new(
                Widget::default()
                    .with_background(Color32::from_rgb(179, 128, 0))
                    .with_height(30.)
                    .with_width(100.),
            )
            .with_text("切换回去")
            .toUi(),
            Button::new(
                Widget::default()
                    .with_background(Color32::from_rgb(179, 128, 0))
                    .with_height(30.)
                    .with_width(100.),
            )
            .with_text("切换回去")
            .toUi(),
        ])
        .build(&mut self.canvas);
    }
}

impl PanelWindow for TestPanel {
    fn get_canvas(&mut self) -> &mut Canvas {
        &mut self.canvas
    }

    fn get_size(&self) -> Vec2 {
        Vec2::INFINITY
    }
}
