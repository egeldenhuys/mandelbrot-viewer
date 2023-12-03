use egui::{Color32, Rounding, Shape, Stroke};
use emath::Pos2;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct MandelbrotViewerApp {
    x: u8,
    y: u8,
    zoom: u8,
    pointer_pos: emath::Pos2,
    lines: Vec<Vec<Pos2>>,
    stroke: Stroke,
    canvas_pos: emath::Pos2,
}

impl Default for MandelbrotViewerApp {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            zoom: 1,
            pointer_pos: emath::Pos2 { x: 0.0, y: 0.0 },
            canvas_pos: emath::Pos2 { x: 0.0, y: 0.0 },
            lines: Default::default(),
            stroke: Stroke::new(1.0, Color32::from_rgb(25, 200, 100)),
        }
    }
}

impl MandelbrotViewerApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for MandelbrotViewerApp {
    /// Called by the frame work to save state before shutdown
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left_side_panel").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label(format!("Pointer Pos: {:?}", self.pointer_pos));
                ui.label(format!("Canvas Pos: {:?}", self.pointer_pos));
                if ui.button("Clear Painting").clicked() {
                    self.lines.clear();
                }
            })
        });

        // egui::CentralPanel::default().show(ctx, |ui| {
        //     let (mut response, painter) =
        //         ui.allocate_painter(ui.available_size_before_wrap(), egui::Sense::hover());

        //     // let to_screen = emath::RectTransform::from_to(
        //     //     egui::Rect::from_min_size(egui::Pos2::ZERO, response.rect.),
        //     //     response.rect,
        //     // );
        //     let from_screen = to_screen.inverse();

        //     if let Some(pointer_pos) = response.hover_pos() {
        //         let canvas_pos = from_screen * pointer_pos;
        //         self.can = canvas_pos;

        //         let rect = Shape::Rect(egui::epaint::RectShape {
        //             rect: emath::Rect {
        //                 min: Pos2::new(1.0, 1.0),
        //                 max: Pos2::new(5.0, 100.0),
        //             },
        //             rounding: Rounding::ZERO,
        //             fill: Color32::RED,
        //             stroke: egui::Stroke {
        //                 width: 1.0,
        //                 color: Color32::BLUE,
        //             },
        //             fill_texture_id: egui::TextureId::Managed(0),
        //             uv: egui::Rect::ZERO,
        //         });

        //         painter.add(rect);
        //     }
        // });
        egui::CentralPanel::default().show(ctx, |ui| {
            let (mut response, painter) =
                ui.allocate_painter(ui.available_size_before_wrap(), egui::Sense::hover());

            let to_screen = emath::RectTransform::from_to(
                egui::Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
                response.rect,
            );
            let from_screen = to_screen.inverse();

            if self.lines.is_empty() {
                self.lines.push(vec![]);
            }

            let current_line = self.lines.last_mut().unwrap();

            if let Some(pointer_pos) = response.hover_pos() {
                let canvas_pos = from_screen * pointer_pos;

                self.pointer_pos = pointer_pos;
                self.canvas_pos = canvas_pos;

                let rect = Shape::Rect(egui::epaint::RectShape {
                    rect: emath::Rect {
                        min: Pos2::new(pointer_pos.x - 5.0, pointer_pos.y - 5.0),
                        max: Pos2::new(pointer_pos.x + 5.0, pointer_pos.y + 5.0),
                    },
                    rounding: Rounding::ZERO,
                    fill: Color32::RED,
                    stroke: egui::Stroke {
                        width: 1.0,
                        color: Color32::BLUE,
                    },
                    fill_texture_id: egui::TextureId::Managed(0),
                    uv: egui::Rect::ZERO,
                });

                painter.add(rect);

                if current_line.last() != Some(&canvas_pos) {
                    current_line.push(canvas_pos);
                    response.mark_changed();
                }
            } else if !current_line.is_empty() {
                self.lines.push(vec![]);
                response.mark_changed();
            }

            let shapes = self
                .lines
                .iter()
                .filter(|line| line.len() >= 2)
                .map(|line| {
                    let points: Vec<Pos2> = line.iter().map(|p| to_screen * *p).collect();
                    egui::Shape::line(points, self.stroke)
                });

            painter.extend(shapes);

            let rect = Shape::Rect(egui::epaint::RectShape {
                rect: emath::Rect {
                    min: Pos2::new(700.0, 100.0),
                    max: Pos2::new(800.0, 200.0),
                },
                rounding: Rounding::ZERO,
                fill: Color32::RED,
                stroke: egui::Stroke {
                    width: 1.0,
                    color: Color32::BLUE,
                },
                fill_texture_id: egui::TextureId::Managed(0),
                uv: egui::Rect::ZERO,
            });

            painter.add(rect);
        });
    }
}
