use std::sync::Arc;
use eframe::egui;
use egui::{Color32, Frame, Id, vec2};

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Drag Test",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(DragApp::default()))),
    )
}

struct DragApp {
    left_clips: Vec<String>,
    right_clips: Vec<String>,
}

impl Default for DragApp {
    fn default() -> Self {
        Self {
            left_clips: vec![
                "Kick.wav".into(),
                "Snare.wav".into(),
                "HiHat.wav".into(),
            ],
            right_clips: vec![
                "Bass.wav".into(),
                "Lead.wav".into(),
            ],
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct DragPayload {
    from_panel: usize, // 0 = left, 1 = right
    from_idx: usize,
}

impl eframe::App for DragApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            let mut drag_action: Option<(Arc<DragPayload>, i32, usize)> = None;

            ui.columns(2, |columns| {

                // LEFT PANEL - Sound Library
                let left_ui = &mut columns[0];
                left_ui.vertical(|ui| {
                    ui.heading("📁 Sound Library");
                    ui.separator();

                    let frame = Frame::default()
                        .fill(Color32::from_rgb(40, 40, 50))
                        .inner_margin(8.0);

                    let (_, dropped) = ui.dnd_drop_zone::<DragPayload, ()>(frame, |ui| {
                        ui.set_min_size(vec2(200.0, 300.0));

                        for (i, clip) in self.left_clips.clone().iter().enumerate() {
                            let id = Id::new(("left", i));
                            let payload = DragPayload {
                                from_panel: 0,
                                from_idx: i,
                            };

                            let response = ui.dnd_drag_source(id, payload, |ui| {
                                ui.horizontal(|ui| {
                                    ui.colored_label(Color32::LIGHT_BLUE, "🎵");
                                    ui.label(clip);
                                });
                            }).response;

                            // Show insertion line and handle drop
                            if let (Some(pointer), Some(hovered)) = (
                                ui.input(|i| i.pointer.interact_pos()),
                                response.dnd_hover_payload::<DragPayload>(),
                            ) {
                                let rect = response.rect;
                                let stroke = egui::Stroke::new(2.0, Color32::YELLOW);

                                let insert_idx = if pointer.y < rect.center().y {
                                    ui.painter().hline(rect.x_range(), rect.top(), stroke);
                                    i
                                } else {
                                    ui.painter().hline(rect.x_range(), rect.bottom(), stroke);
                                    i + 1
                                };

                                if let Some(dropped_payload) = response.dnd_release_payload::<DragPayload>() {
                                    drag_action = Some((dropped_payload, 0, insert_idx));
                                }
                            }
                        }

                        if self.left_clips.is_empty() {
                            ui.centered_and_justified(|ui| {
                                ui.label("Drop clips here");
                            });
                        }
                    });

                    // Handle drop into empty zone
                    if let Some(payload) = dropped {
                        drag_action = Some((payload, 0, self.left_clips.len()));
                    }
                });



                // RIGHT PANEL - Playlist
                let right_ui = &mut columns[1];
                right_ui.vertical(|ui| {
                    ui.heading("🎼 Playlist");
                    ui.separator();

                    let frame = Frame::default()
                        .fill(Color32::from_rgb(50, 40, 40))
                        .inner_margin(8.0);

                    let (_, dropped) = ui.dnd_drop_zone::<DragPayload, ()>(frame, |ui| {
                        ui.set_min_size(vec2(200.0, 300.0));

                        for (i, clip) in self.right_clips.clone().iter().enumerate() {
                            let id = Id::new(("right", i));
                            let payload = DragPayload {
                                from_panel: 1,
                                from_idx: i,
                            };

                            let response = ui.dnd_drag_source(id, payload, |ui| {
                                ui.horizontal(|ui| {
                                    ui.colored_label(Color32::LIGHT_GREEN, "▶");
                                    ui.label(clip);
                                });
                            }).response;

                            // Show insertion line and handle drop
                            if let (Some(pointer), Some(hovered)) = (
                                ui.input(|i| i.pointer.interact_pos()),
                                response.dnd_hover_payload::<DragPayload>(),
                            ) {
                                let rect = response.rect;
                                let stroke = egui::Stroke::new(2.0, Color32::GREEN);

                                let insert_idx = if pointer.y < rect.center().y {
                                    ui.painter().hline(rect.x_range(), rect.top(), stroke);
                                    i
                                } else {
                                    ui.painter().hline(rect.x_range(), rect.bottom(), stroke);
                                    i + 1
                                };

                                if let Some(dropped_payload) = response.dnd_release_payload::<DragPayload>() {
                                    drag_action = Some((dropped_payload, 1, insert_idx));
                                }
                            }
                        }

                        if self.right_clips.is_empty() {
                            ui.centered_and_justified(|ui| {
                                ui.label("Drop clips here");
                            });
                        }
                    });

                    // Handle drop into empty zone
                    if let Some(payload) = dropped {
                        drag_action = Some((payload, 1, self.right_clips.len()));
                    }
                });
            }); // end columns

            // Execute the drag action
            if let Some((from, to_panel, to_idx)) = drag_action {


                let from_list = if from.from_panel == 0 {
                    &mut self.left_clips
                } else {
                    &mut self.right_clips
                };



                let item = from_list.remove(from.from_idx);



                let to_list = if to_panel == 0 {
                    &mut self.left_clips
                } else {
                    &mut self.right_clips
                };


                // change order in rack
                let insert_pos = to_idx.min(to_list.len());
                to_list.insert(insert_pos, item);



            }
        });
    }
}