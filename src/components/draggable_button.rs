use egui::{Color32, Id, Pos2, Rect, Response, Sense, Ui, Vec2};

pub struct DraggableButton {
    pub handle_response: Response,
    pub name_response: Response,
    pub handle_id: Id,
}

pub fn draw_draggable_button(
    ui: &mut Ui,
    id: Id,
    total_width: f32,
    height: f32,
    name: &str,
    bg_color: Color32,
    bg_hover_color: Color32,
    handle_color: Color32,
    handle_hover_color: Color32,
    text_color: Color32,
) -> DraggableButton {
    let handle_width = 20.0;
    let name_width = total_width - handle_width;

    let (full_rect, _) = ui.allocate_exact_size(Vec2::new(total_width, height), Sense::hover());

    let handle_rect = Rect::from_min_size(full_rect.min, Vec2::new(handle_width, height));
    let name_rect = Rect::from_min_size(
        Pos2::new(full_rect.min.x + handle_width, full_rect.min.y),
        Vec2::new(name_width, height),
    );

    let name_id = id.with("_name");
    let name_response = ui.interact(name_rect, name_id, Sense::click());

    let handle_id = id.with("_handle");
    let handle_response = ui.interact(handle_rect, handle_id, Sense::click_and_drag());

    // Draw background
    let final_bg_color = if name_response.hovered() {
        bg_hover_color
    } else {
        bg_color
    };
    ui.painter().rect_filled(full_rect, 3.0, final_bg_color);

    // Draw handle dots
    let final_handle_color = if handle_response.hovered() {
        handle_hover_color
    } else {
        handle_color
    };

    draw_drag_dots(ui, handle_rect, final_handle_color);

    // Draw separator line
    ui.painter().vline(
        handle_rect.right(),
        full_rect.y_range(),
        egui::Stroke::new(1.0, Color32::from_gray(80)),
    );

    // Draw text
    ui.painter().text(
        name_rect.center(),
        egui::Align2::CENTER_CENTER,
        name,
        egui::FontId::default(),
        text_color,
    );

    DraggableButton {
        handle_response,
        name_response,
        handle_id,
    }
}

fn draw_drag_dots(ui: &mut Ui, handle_rect: Rect, handle_color: Color32) {
    let center = handle_rect.center();
    let dot_radius = 1.25;
    let spacing_x = 3.5;
    let spacing_y = 5.5;

    let start_x = center.x - spacing_x;
    let start_y = center.y - spacing_y;

    for row in 0..3 {
        for col in 0..3 {
            let x = start_x + col as f32 * spacing_x;
            let y = start_y + row as f32 * spacing_y;
            ui.painter()
                .circle_filled(Pos2::new(x, y), dot_radius, handle_color);
        }
    }
}