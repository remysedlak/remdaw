use crate::models::MyApp;

pub fn render(app: &mut MyApp, ctx: &egui::Context) {
    ctx.request_repaint();

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.label(egui::RichText::new("Playlist").strong().size(20.0));
        ui.separator();

        let mut state = app.audio_state.lock().unwrap();
        let playlist = &mut state.playlist;

        let (response, painter) = ui.allocate_painter(
            egui::Vec2::new(ui.available_width(), 400.0),
            egui::Sense::click_and_drag()
        );

        let rect = response.rect;

        // Check if any drag just ended (anywhere on screen)
        let pointer_released = ctx.input(|i| i.pointer.any_released());

        if pointer_released {
            if let Some(pointer_pos) = ctx.pointer_interact_pos() {
                // Check if a pattern is being dragged
                if let Some(pattern_idx) = ctx.memory(|mem| {
                    mem.data.get_temp::<usize>(egui::Id::new("dragging_pattern"))
                }) {
                    println!("Pattern {} being dropped at {:?}", pattern_idx, pointer_pos);

                    // Check if pointer is over the playlist area
                    if rect.contains(pointer_pos) {
                        println!("Drop is inside playlist!");

                        // Calculate which track and time position
                        let timeline_start_x = rect.left() + 150.0;
                        let tracks_start_y = rect.top() + 50.0;
                        let pixels_per_beat = 100.0;

                        // Find track index
                        let relative_y = pointer_pos.y - tracks_start_y;
                        let track_idx = (relative_y / 60.0).floor() as usize;

                        // Calculate start time in beats
                        let relative_x = pointer_pos.x - timeline_start_x;
                        let start_beat = (relative_x / pixels_per_beat).max(0.0);

                        // Snap to nearest beat
                        let snapped_beat = start_beat.round();

                        println!("Track: {}, Beat: {}", track_idx, snapped_beat);

                        if track_idx < playlist.tracks.len() {
                            // Add the clip
                            playlist.clips.push(crate::models::PlacedClip {
                                pattern_id: pattern_idx,
                                track_index: track_idx,
                                start_time: snapped_beat as f64,
                                length: 4.0,
                                color: egui::Color32::from_rgb(80, 120, 200),
                            });
                            println!("Clip added! Total clips: {}", playlist.clips.len());
                        }
                    }

                    // Clear the drag state
                    ctx.memory_mut(|mem| {
                        mem.data.remove::<usize>(egui::Id::new("dragging_pattern"));
                    });
                }
            }
        }

        let beats_per_bar = 4;
        let pixels_per_beat = 100.0;
        let timeline_start_x = rect.left() + 150.0;

        // Draw timeline header background
        painter.rect_filled(
            egui::Rect::from_min_size(
                egui::pos2(timeline_start_x, rect.top()),
                egui::vec2(rect.width() - 150.0, 40.0)
            ),
            0.0,
            egui::Color32::from_gray(30)
        );

        // Draw beat markers
        for beat in 0..40 {
            let x = timeline_start_x + (beat as f32 * pixels_per_beat);

            if x > rect.right() {
                break;
            }

            let is_bar = beat % beats_per_bar == 0;

            let tick_height = if is_bar { 30.0 } else { 15.0 };
            let color = if is_bar {
                egui::Color32::WHITE
            } else {
                egui::Color32::from_gray(120)
            };

            painter.line_segment(
                [
                    egui::pos2(x, rect.top()),
                    egui::pos2(x, rect.top() + tick_height)
                ],
                egui::Stroke::new(if is_bar { 2.0 } else { 1.0 }, color)
            );

            if is_bar {
                let bar_number = (beat / beats_per_bar) + 1;
                painter.text(
                    egui::pos2(x + 3.0, rect.top() + 2.0),
                    egui::Align2::LEFT_TOP,
                    format!("{}", bar_number),
                    egui::FontId::proportional(14.0),
                    egui::Color32::WHITE
                );
            }
        }

        // Draw tracks
        let tracks_start_y = rect.top() + 50.0;
        for (idx, track) in playlist.tracks.iter().enumerate() {
            let y = tracks_start_y + idx as f32 * track.height;

            painter.rect_filled(
                egui::Rect::from_min_size(
                    egui::pos2(rect.left(), y),
                    egui::vec2(rect.width(), track.height)
                ),
                0.0,
                if idx % 2 == 0 {
                    egui::Color32::from_gray(40)
                } else {
                    egui::Color32::from_gray(50)
                }
            );

            painter.text(
                egui::pos2(rect.left() + 5.0, y + track.height / 2.0),
                egui::Align2::LEFT_CENTER,
                &track.name,
                egui::FontId::default(),
                egui::Color32::WHITE
            );
        }

        // Draw clips
        for clip in &playlist.clips {
            let track = &playlist.tracks[clip.track_index];
            let y = tracks_start_y + clip.track_index as f32 * track.height;

            let x = timeline_start_x + (clip.start_time as f32 * pixels_per_beat);
            let width = clip.length as f32 * pixels_per_beat;

            painter.rect_filled(
                egui::Rect::from_min_size(
                    egui::pos2(x, y + 5.0),
                    egui::vec2(width, track.height - 10.0)
                ),
                5.0,
                clip.color
            );

            painter.text(
                egui::pos2(x + 5.0, y + track.height / 2.0),
                egui::Align2::LEFT_CENTER,
                format!("Pattern {}", clip.pattern_id),
                egui::FontId::default(),
                egui::Color32::WHITE
            );
        }

        // Draw playhead
        let playhead_x = timeline_start_x + (state.playhead_position as f32 * pixels_per_beat);
        painter.vline(
            playhead_x,
            rect.top()..=rect.bottom(),
            egui::Stroke::new(3.0, egui::Color32::RED)
        );
    });
}