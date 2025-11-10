use std::path::PathBuf;
use eframe::epaint::{Color32, Stroke};
use crate::models::{MyApp, ClipType, ResizeEdge, ResizeState};

pub fn render(app: &mut MyApp, ctx: &egui::Context) {
    ctx.request_repaint();

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.label(egui::RichText::new("Playlist").strong().size(20.0));
        ui.separator();

        let (response, painter) = ui.allocate_painter(
            egui::Vec2::new(ui.available_width(), app.ui_state.playlist_height),
            egui::Sense::click_and_drag(),
        );

        let rect = response.rect;
        let pointer_pos = ctx.pointer_interact_pos();

        const EDGE_GRAB_DISTANCE: f32 = 8.0;

        // Check if pointer button just pressed or released
        let pointer_pressed = ctx.input(|i| i.pointer.primary_pressed());
        let pointer_released = ctx.input(|i| i.pointer.any_released());

        // Handle resize drag ending
        if pointer_released && app.ui_state.resizing_clip.is_some() {
            println!("Resize ended");
            app.ui_state.resizing_clip = None;
        }

        if pointer_released {
            if let Some(pointer_pos) = ctx.pointer_interact_pos() {
                // Check for pattern being dragged
                if let Some(pattern_idx) = ctx.memory(|mem| {
                    mem.data.get_temp::<usize>(egui::Id::new("dragging_pattern"))
                }) {
                    println!("Pattern {} being dropped at {:?}", pattern_idx, pointer_pos);

                    if rect.contains(pointer_pos) {
                        println!("Drop is inside playlist!");

                        let timeline_start_x = rect.left() + 150.0;
                        let tracks_start_y = rect.top() + 50.0;
                        let pixels_per_beat = 100.0;

                        let relative_y = pointer_pos.y - tracks_start_y;
                        let track_idx = (relative_y / 60.0).floor() as usize;

                        let relative_x = pointer_pos.x - timeline_start_x;
                        let start_beat = (relative_x / pixels_per_beat).max(0.0).round();

                        println!("Track: {}, Beat: {}", track_idx, start_beat);

                        let mut state = app.audio_state.lock().unwrap();
                        if track_idx < state.playlist.tracks.len() {
                            let name = state.patterns.get(pattern_idx)
                                .map(|p| p.name.clone())
                                .unwrap_or_else(|| "Unknown".to_string());

                            state.playlist.clips.push(crate::models::PlacedClip {
                                clip_type: crate::models::ClipType::Pattern(pattern_idx),
                                track_index: track_idx,
                                start_time: start_beat as f64,
                                name,
                                length: 4.0,
                                color: Color32::from_rgb(80, 120, 200),
                            });
                            println!("Pattern clip added! Total clips: {}", state.playlist.clips.len());
                        }
                    }

                    ctx.memory_mut(|mem| {
                        mem.data.remove::<usize>(egui::Id::new("dragging_pattern"));
                    });
                }
                // Check for audio file being dragged
                else if let Some(file_path) = ctx.memory(|mem| {
                    mem.data.get_temp::<PathBuf>(egui::Id::new("dragging_audio_file"))
                }) {
                    println!("Audio file being dropped: {:?}", file_path);

                    if rect.contains(pointer_pos) {
                        println!("Drop is inside playlist!");

                        let timeline_start_x = rect.left() + 150.0;
                        let tracks_start_y = rect.top() + 50.0;
                        let pixels_per_beat = 100.0;

                        let relative_y = pointer_pos.y - tracks_start_y;
                        let track_idx = (relative_y / 60.0).floor() as usize;

                        let relative_x = pointer_pos.x - timeline_start_x;
                        let start_beat = (relative_x / pixels_per_beat).max(0.0).round();

                        let mut state = app.audio_state.lock().unwrap();
                        if track_idx < state.playlist.tracks.len() {
                            // Load the audio file and add as instrument
                            let samples = crate::audio::path_to_vector(file_path.to_str().unwrap());
                            let name = file_path.file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("Unknown")
                                .to_string();

                            // Add to instruments
                            state.instruments.push(crate::models::Instrument {
                                name: name.clone(),
                                file_path: file_path.clone(),
                                samples,
                                position: 0,
                                is_playing: false,
                            });
                            state.pattern.push(vec![false; 16]);

                            let instrument_idx = state.instruments.len() - 1;

                            // Add clip to playlist
                            state.playlist.clips.push(crate::models::PlacedClip {
                                clip_type: crate::models::ClipType::AudioFile(instrument_idx),
                                track_index: track_idx,
                                start_time: start_beat as f64,
                                name: name.clone(),
                                length: 4.0,
                                color: Color32::from_rgb(200, 120, 80),
                            });

                            println!("Audio clip added!");
                        }
                    }

                    ctx.memory_mut(|mem| {
                        mem.data.remove::<PathBuf>(egui::Id::new("dragging_audio_file"));
                    });
                }
            }
        }

        // Lock state for drawing (immutable borrow)
        let state = app.audio_state.lock().unwrap();
        let playlist = &state.playlist;

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
            Color32::from_gray(30)
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
                Color32::WHITE
            } else {
                Color32::from_gray(120)
            };

            painter.line_segment(
                [
                    egui::pos2(x, rect.top()),
                    egui::pos2(x, rect.top() + tick_height)
                ],
                Stroke::new(if is_bar { 2.0 } else { 1.0 }, color)
            );

            if is_bar {
                let bar_number = (beat / beats_per_bar) + 1;
                painter.text(
                    egui::pos2(x + 3.0, rect.top() + 2.0),
                    egui::Align2::LEFT_TOP,
                    format!("{}", bar_number),
                    egui::FontId::proportional(14.0),
                    Color32::WHITE
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
                    Color32::from_gray(40)
                } else {
                    Color32::from_gray(50)
                }
            );

            // mute button
            painter.circle(
                egui::Pos2 { x: rect.left(), y: y + track.height / 2.0 },
                10.0,
                Color32::from_rgb(155, 0, 0),
                Stroke::new(1.0, Color32::WHITE)
            );

            painter.text(
                egui::pos2(rect.left() + 5.0, y + track.height / 2.0),
                egui::Align2::LEFT_CENTER,
                &track.name,
                egui::FontId::default(),
                Color32::WHITE
            );
        }

        // Track which edge we're hovering over
        let mut hovered_edge: Option<(usize, ResizeEdge)> = None;

        // Draw clips WITH resize detection
        for (clip_idx, clip) in playlist.clips.iter().enumerate() {
            let track = &playlist.tracks[clip.track_index];
            let y = tracks_start_y + clip.track_index as f32 * track.height;

            let x = timeline_start_x + (clip.start_time as f32 * pixels_per_beat);
            let width = clip.length as f32 * pixels_per_beat;

            let clip_rect = egui::Rect::from_min_size(
                egui::pos2(x, y + 5.0),
                egui::vec2(width, track.height - 10.0)
            );

            // Check if we're hovering near edges (only for Pattern clips)
            if let Some(pos) = pointer_pos {
                if matches!(clip.clip_type, ClipType::Pattern(_)) && clip_rect.contains(pos) {
                    let dist_to_left = (pos.x - clip_rect.left()).abs();
                    let dist_to_right = (pos.x - clip_rect.right()).abs();

                    if dist_to_left < EDGE_GRAB_DISTANCE {
                        hovered_edge = Some((clip_idx, ResizeEdge::Left));
                    } else if dist_to_right < EDGE_GRAB_DISTANCE {
                        hovered_edge = Some((clip_idx, ResizeEdge::Right));
                    }
                }
            }

            painter.rect_filled(clip_rect, 5.0, clip.color);

            painter.text(
                egui::pos2(x + 5.0, y + track.height / 2.0),
                egui::Align2::LEFT_CENTER,
                format!("{}", clip.name),
                egui::FontId::default(),
                Color32::WHITE
            );
        }

        // Draw playhead
        let playhead_x = timeline_start_x + (state.playhead_position as f32 * pixels_per_beat);
        painter.vline(
            playhead_x,
            rect.top()..=rect.bottom(),
            Stroke::new(3.0, Color32::RED)
        );

        drop(state); // Release the lock before modifying

        // Update cursor based on hover
        if hovered_edge.is_some() {
            ctx.set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
        }

        // Handle resize drag starting - check when pointer is pressed AND we're hovering an edge
        if pointer_pressed && hovered_edge.is_some() && app.ui_state.resizing_clip.is_none() {
            if let Some((clip_idx, edge)) = hovered_edge {
                println!("Started resizing clip {} on edge {:?}", clip_idx, match edge {
                    ResizeEdge::Left => "Left",
                    ResizeEdge::Right => "Right",
                });

                let state = app.audio_state.lock().unwrap();
                let clip = &state.playlist.clips[clip_idx];

                app.ui_state.resizing_clip = Some(ResizeState {
                    clip_index: clip_idx,
                    edge,
                    initial_start: clip.start_time,
                    initial_length: clip.length,
                });
            }
        }

        // Handle active resizing - check if we're currently in a resize state and pointer is down
        // Handle active resizing - check if we're currently in a resize state and pointer is down
        // Handle active resizing - check if we're currently in a resize state and pointer is down
        if ctx.input(|i| i.pointer.primary_down()) {
            if let Some(resize_state) = &app.ui_state.resizing_clip {
                let drag_delta = ctx.input(|i| i.pointer.delta());

                if drag_delta.x.abs() > 0.01 { // Only update if there's actual movement
                    let delta_beats = drag_delta.x / pixels_per_beat;

                    println!("Resizing: delta_beats = {}", delta_beats);

                    let mut state = app.audio_state.lock().unwrap();
                    let clip = &mut state.playlist.clips[resize_state.clip_index];

                    match resize_state.edge {
                        ResizeEdge::Left => {
                            // Dragging left edge: change start and adjust length
                            let new_start = clip.start_time + delta_beats as f64;
                            let new_length = clip.length - delta_beats as f64;

                            println!("Left edge: new_start = {}, new_length = {}", new_start, new_length);

                            // Minimum length of 0.25 beats
                            if new_length > 0.25 && new_start >= 0.0 {
                                // Apply snapping if enabled
                                if app.ui_state.snap_to_grid {
                                    let snap_div = app.ui_state.snap_division as f64;
                                    clip.start_time = (new_start / snap_div).round() * snap_div;
                                    clip.length = (new_length / snap_div).round() * snap_div;
                                } else {
                                    clip.start_time = new_start;
                                    clip.length = new_length;
                                }
                            }
                        }
                        ResizeEdge::Right => {
                            // Dragging right edge: only change length
                            let new_length = clip.length + delta_beats as f64;

                            println!("Right edge: new_length = {}", new_length);

                            if new_length > 0.25 {
                                // Apply snapping if enabled
                                if app.ui_state.snap_to_grid {
                                    let snap_div = app.ui_state.snap_division as f64;
                                    clip.length = (new_length / snap_div).round() * snap_div;
                                } else {
                                    clip.length = new_length;
                                }
                            }
                        }
                    }
                }
            }
        }
    });
}