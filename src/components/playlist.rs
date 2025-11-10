use std::path::PathBuf;
use eframe::emath::Align::Center;
use eframe::epaint::{Color32, Stroke};
use crate::components::snap_to_grid;
use crate::models::{MyApp, ClipType, ResizeEdge, ResizeState};

/// Renders the playlist view where clips are arranged on tracks over time
pub fn render(app: &mut MyApp, ctx: &egui::Context) {
    // Request continuous repainting for smooth playhead animation
    ctx.request_repaint();

    egui::CentralPanel::default().show(ctx, |ui| {

        // === TITLE BAR ===
        // Horizontal layout with title on left, controls on right
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Playlist").strong().size(20.0));
            // Right-aligned snap controls
            ui.with_layout(egui::Layout::right_to_left(Center), |ui| {
                snap_to_grid::render(ui, app);
            });
        });

        ui.separator();

        // === SET UP PAINTER ===
        // Allocate a drawing area for the playlist
        // Sense::click_and_drag() enables mouse interaction
        let (response, painter) = ui.allocate_painter(
            egui::Vec2::new(ui.available_width(), app.ui_state.playlist_height),
            egui::Sense::click_and_drag(),
        );

        let rect = response.rect; // The bounding box of the playlist area
        let pointer_pos = ctx.pointer_interact_pos(); // Current mouse position (if hovering)

        // How close to an edge the mouse must be to trigger resize mode (in pixels)
        const EDGE_GRAB_DISTANCE: f32 = 8.0;

        // === MOUSE INPUT DETECTION ===
        let pointer_pressed = ctx.input(|i| i.pointer.primary_pressed()); // Left click just pressed
        let pointer_released = ctx.input(|i| i.pointer.any_released()); // Any mouse button released

        // === HANDLE RESIZE ENDING ===
        // When mouse is released and we were resizing, stop resizing
        if pointer_released && app.ui_state.resizing_clip.is_some() {
            println!("Resize ended");
            app.ui_state.resizing_clip = None;
        }

        // === HANDLE DRAG-AND-DROP FROM SIDE PANELS ===
        if pointer_released {
            if let Some(pointer_pos) = ctx.pointer_interact_pos() {

                // --- Check for PATTERN being dropped ---
                if let Some(pattern_idx) = ctx.memory(|mem| {
                    mem.data.get_temp::<usize>(egui::Id::new("dragging_pattern"))
                }) {
                    println!("Pattern {} being dropped at {:?}", pattern_idx, pointer_pos);

                    // Only add clip if dropped inside the playlist area
                    if rect.contains(pointer_pos) {
                        println!("Drop is inside playlist!");

                        // Calculate grid position from mouse position
                        let timeline_start_x = rect.left() + 150.0; // Left margin for track labels
                        let tracks_start_y = rect.top() + 50.0; // Top margin for timeline
                        let pixels_per_beat = 100.0; // Visual scale

                        // Which track? (vertical position)
                        let relative_y = pointer_pos.y - tracks_start_y;
                        let track_idx = (relative_y / 60.0).floor() as usize; // 60px per track

                        // Which beat? (horizontal position)
                        let relative_x = pointer_pos.x - timeline_start_x;
                        let start_beat = (relative_x / pixels_per_beat).max(0.0).round();

                        println!("Track: {}, Beat: {}", track_idx, start_beat);

                        // Create the clip in the playlist
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
                                length: 4.0, // Default 4 beats long
                                color: Color32::from_rgb(80, 120, 200), // Blue for patterns
                            });
                            println!("Pattern clip added! Total clips: {}", state.playlist.clips.len());
                        }
                    }

                    // Clear the drag state
                    ctx.memory_mut(|mem| {
                        mem.data.remove::<usize>(egui::Id::new("dragging_pattern"));
                    });
                }

                // --- Check for AUDIO FILE being dropped ---
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

                            // Add to instruments list
                            state.instruments.push(crate::models::Instrument {
                                name: name.clone(),
                                file_path: file_path.clone(),
                                samples,
                                position: 0,
                                is_playing: false,
                            });
                            // Add corresponding pattern row (empty)
                            state.pattern.push(vec![false; 16]);

                            let instrument_idx = state.instruments.len() - 1;

                            // Add clip to playlist
                            state.playlist.clips.push(crate::models::PlacedClip {
                                clip_type: crate::models::ClipType::AudioFile(instrument_idx),
                                track_index: track_idx,
                                start_time: start_beat as f64,
                                name: name.clone(),
                                length: 4.0,
                                color: Color32::from_rgb(200, 120, 80), // Orange for audio files
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

        // === LOCK STATE FOR DRAWING ===
        // Lock audio state to read playlist data (immutable borrow)
        let state = app.audio_state.lock().unwrap();
        let playlist = &state.playlist;

        // Grid configuration
        let beats_per_bar = 4; // 4/4 time signature
        let pixels_per_beat = 100.0; // Zoom level (pixels per beat)
        let timeline_start_x = rect.left() + 150.0; // Left margin for track names

        // === DRAW TIMELINE HEADER ===
        // Dark background bar at the top showing beat numbers
        painter.rect_filled(
            egui::Rect::from_min_size(
                egui::pos2(timeline_start_x, rect.top()),
                egui::vec2(rect.width() - 150.0, 40.0)
            ),
            0.0, // No corner rounding
            Color32::from_gray(30) // Dark gray
        );

        // === DRAW BEAT MARKERS ===
        // Vertical lines marking each beat and bar
        for beat in 0..40 {
            let x = timeline_start_x + (beat as f32 * pixels_per_beat);

            // Stop drawing if we're past the right edge
            if x > rect.right() {
                break;
            }

            let is_bar = beat % beats_per_bar == 0; // Every 4th beat is a bar line

            // Bar lines are taller and thicker
            let tick_height = if is_bar { 30.0 } else { 15.0 };
            let color = if is_bar {
                Color32::WHITE
            } else {
                Color32::from_gray(120)
            };

            // Draw the vertical tick
            painter.line_segment(
                [
                    egui::pos2(x, rect.top()),
                    egui::pos2(x, rect.top() + tick_height)
                ],
                Stroke::new(if is_bar { 2.0 } else { 1.0 }, color)
            );

            // Draw bar numbers at bar boundaries
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

        // === DRAW TRACKS ===
        // Horizontal lanes where clips are placed
        let tracks_start_y = rect.top() + 50.0; // Below the timeline header
        for (idx, track) in playlist.tracks.iter().enumerate() {
            let y = tracks_start_y + idx as f32 * track.height;

            // Alternating background colors for visual separation
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

            // Draw mute button (currently non-functional, just visual)
            painter.circle(
                egui::Pos2 { x: rect.left(), y: y + track.height / 2.0 },
                10.0, // Radius
                Color32::from_rgb(155, 0, 0), // Red fill
                Stroke::new(1.0, Color32::WHITE) // White outline
            );

            // Draw track name label
            painter.text(
                egui::pos2(rect.left() + 5.0, y + track.height / 2.0),
                egui::Align2::LEFT_CENTER,
                &track.name,
                egui::FontId::default(),
                Color32::WHITE
            );
        }

        // === DETECT RESIZE HOVER ===
        // Track which clip edge the mouse is hovering over (if any)
        let mut hovered_edge: Option<(usize, ResizeEdge)> = None;

        // === DRAW CLIPS ===
        // Draw each clip as a colored rectangle on its track
        for (clip_idx, clip) in playlist.clips.iter().enumerate() {
            let track = &playlist.tracks[clip.track_index];
            let y = tracks_start_y + clip.track_index as f32 * track.height;

            // Calculate clip rectangle position and size
            let x = timeline_start_x + (clip.start_time as f32 * pixels_per_beat);
            let width = clip.length as f32 * pixels_per_beat;

            let clip_rect = egui::Rect::from_min_size(
                egui::pos2(x, y + 5.0), // Small vertical padding
                egui::vec2(width, track.height - 10.0)
            );

            // Check if mouse is hovering near the left or right edge (only for patterns)
            if let Some(pos) = pointer_pos {
                if matches!(clip.clip_type, ClipType::Pattern(_)) && clip_rect.contains(pos) {
                    let dist_to_left = (pos.x - clip_rect.left()).abs();
                    let dist_to_right = (pos.x - clip_rect.right()).abs();

                    // If within EDGE_GRAB_DISTANCE of an edge, mark it for resize
                    if dist_to_left < EDGE_GRAB_DISTANCE {
                        hovered_edge = Some((clip_idx, ResizeEdge::Left));
                    } else if dist_to_right < EDGE_GRAB_DISTANCE {
                        hovered_edge = Some((clip_idx, ResizeEdge::Right));
                    }
                }
            }

            // Draw the clip rectangle
            painter.rect_filled(
                clip_rect,
                5.0, // Corner rounding
                clip.color // Pattern clips are blue, audio files are orange
            );

            // Draw the clip name label
            painter.text(
                egui::pos2(x + 5.0, y + track.height / 2.0),
                egui::Align2::LEFT_CENTER,
                format!("{}", clip.name),
                egui::FontId::default(),
                Color32::WHITE
            );
        }

        // === DRAW PLAYHEAD ===
        // Red vertical line showing current playback position
        let playhead_x = timeline_start_x + (state.playhead_position as f32 * pixels_per_beat);
        painter.vline(
            playhead_x,
            rect.top()..=rect.bottom(), // Full height
            Stroke::new(3.0, Color32::RED) // Thick red line
        );

        // Release the lock before modifying state
        drop(state);

        // === UPDATE CURSOR ===
        // Show resize cursor when hovering over a clip edge
        if hovered_edge.is_some() {
            ctx.set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
        }

        // === START RESIZE OPERATION ===
        // When mouse is pressed while hovering an edge, begin resizing
        if pointer_pressed && hovered_edge.is_some() && app.ui_state.resizing_clip.is_none() {
            if let Some((clip_idx, edge)) = hovered_edge {
                println!("Started resizing clip {} on edge {:?}", clip_idx, match edge {
                    ResizeEdge::Left => "Left",
                    ResizeEdge::Right => "Right",
                });

                let state = app.audio_state.lock().unwrap();
                let clip = &state.playlist.clips[clip_idx];

                // Store resize state: which clip, which edge, and initial values
                app.ui_state.resizing_clip = Some(ResizeState {
                    clip_index: clip_idx,
                    edge,
                    initial_start: clip.start_time,
                    initial_length: clip.length,
                });
            }
        }

        // === PERFORM RESIZE ===
        // While mouse is held down and we're in resize mode, update clip position/length
        if ctx.input(|i| i.pointer.primary_down()) {
            if let Some(resize_state) = &app.ui_state.resizing_clip {
                let drag_delta = ctx.input(|i| i.pointer.delta()); // How much mouse moved this frame

                // Only update if there's actual movement (avoid jitter)
                if drag_delta.x.abs() > 0.01 {
                    // Convert pixel movement to beats
                    let delta_beats = drag_delta.x / pixels_per_beat;

                    println!("Resizing: delta_beats = {}", delta_beats);

                    let mut state = app.audio_state.lock().unwrap();
                    let clip = &mut state.playlist.clips[resize_state.clip_index];

                    match resize_state.edge {
                        ResizeEdge::Left => {
                            // LEFT EDGE: moving left edge changes both start position AND length
                            // (like trimming the beginning of a video clip)
                            let new_start = clip.start_time + delta_beats as f64;
                            let new_length = clip.length - delta_beats as f64;

                            println!("Left edge: new_start = {}, new_length = {}", new_start, new_length);

                            // Enforce minimum length and don't go before beat 0
                            if new_length > 0.25 && new_start >= 0.0 {
                                // Apply grid snapping if enabled
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
                            // RIGHT EDGE: moving right edge only changes length
                            // (like extending or shortening a clip)
                            let new_length = clip.length + delta_beats as f64;

                            println!("Right edge: new_length = {}", new_length);

                            // Enforce minimum length
                            if new_length > 0.25 {
                                // Apply grid snapping if enabled
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