// src/components/playlist/input.rs

use egui::{Context, Response, Rect, Pos2, CursorIcon};
use crate::models::MyApp;
use super::config::PlaylistConfig;
use super::{drag_drop, resize};

/// Main input handling function
pub fn handle_input(
    app: &mut MyApp,
    ctx: &Context,
    response: &Response,
    rect: Rect,
    pointer_pos: Option<Pos2>,
    config: &PlaylistConfig,
) {
    let pointer_pressed = ctx.input(|i| i.pointer.primary_pressed());
    let pointer_released = ctx.input(|i| i.pointer.any_released());
    let pointer_down = ctx.input(|i| i.pointer.primary_down());

    // Handle resize ending
    if pointer_released && app.ui_state.resizing_clip.is_some() {
        resize::end_resize(app);
    }

    // Handle drag and drop ending
    if pointer_released {
        if let Some(pointer_pos) = pointer_pos {
            drag_drop::handle_pattern_drop(app, ctx, pointer_pos, rect, config);
            drag_drop::handle_audio_drop(app, ctx, pointer_pos, rect, config);
        }
    }

    // Detect resize hover
    let hovered_edge = resize::detect_resize_hover(pointer_pos, app, rect, config);

    // Update cursor
    if hovered_edge.is_some() {
        ctx.set_cursor_icon(CursorIcon::ResizeHorizontal);
    }

    // Start resize
    if pointer_pressed && hovered_edge.is_some() && app.ui_state.resizing_clip.is_none() {
        if let Some((clip_idx, edge)) = hovered_edge {
            resize::start_resize(app, clip_idx, edge);
        }
    }

    // Perform resize
    if pointer_down && app.ui_state.resizing_clip.is_some() {
        let drag_delta = ctx.input(|i| i.pointer.delta());
        resize::perform_resize(app, drag_delta, config);
    }
}