use egui::Color32;

#[derive(Clone, Debug)]
pub struct Theme {
    // Background colors
    pub bg_primary: Color32,
    pub bg_secondary: Color32,
    pub bg_tertiary: Color32,

    // Panel colors
    pub panel_bg: Color32,
    pub panel_header: Color32,

    // Pattern colors
    pub pattern_bg: Color32,
    pub pattern_bg_hover: Color32,
    pub pattern_handle: Color32,
    pub pattern_handle_hover: Color32,
    pub pattern_text: Color32,

    // Playlist colors
    pub playlist_bg: Color32,
    pub track_even: Color32,
    pub track_odd: Color32,
    pub clip_default: Color32,
    pub clip_pattern: Color32,
    pub clip_audio: Color32,

    // UI elements
    pub text_primary: Color32,
    pub text_secondary: Color32,
    pub border: Color32,
    pub accent: Color32,
    pub playhead: Color32,
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            bg_primary: Color32::from_rgb(30, 30, 35),
            bg_secondary: Color32::from_rgb(40, 40, 45),
            bg_tertiary: Color32::from_rgb(50, 50, 55),

            panel_bg: Color32::from_rgb(35, 35, 40),
            panel_header: Color32::from_rgb(45, 45, 50),

            pattern_bg: Color32::from_rgb(60, 60, 80),
            pattern_bg_hover: Color32::from_rgb(70, 70, 90),
            pattern_handle: Color32::from_rgb(120, 120, 140),
            pattern_handle_hover: Color32::from_rgb(150, 150, 170),
            pattern_text: Color32::WHITE,

            playlist_bg: Color32::from_rgb(25, 25, 30),
            track_even: Color32::from_rgb(35, 35, 40),
            track_odd: Color32::from_rgb(40, 40, 45),
            clip_default: Color32::from_rgb(100, 150, 200),
            clip_pattern: Color32::from_rgb(150, 100, 200),
            clip_audio: Color32::from_rgb(100, 200, 150),

            text_primary: Color32::WHITE,
            text_secondary: Color32::from_rgb(180, 180, 180),
            border: Color32::from_rgb(80, 80, 100),
            accent: Color32::from_rgb(100, 150, 255),
            playhead: Color32::from_rgb(255, 100, 100),
        }
    }

    pub fn light() -> Self {
        Self {
            bg_primary: Color32::from_rgb(240, 240, 245),
            bg_secondary: Color32::from_rgb(230, 230, 235),
            bg_tertiary: Color32::from_rgb(220, 220, 225),

            panel_bg: Color32::from_rgb(250, 250, 255),
            panel_header: Color32::from_rgb(240, 240, 250),

            pattern_bg: Color32::from_rgb(200, 200, 220),
            pattern_bg_hover: Color32::from_rgb(180, 180, 210),
            pattern_handle: Color32::from_rgb(100, 100, 120),
            pattern_handle_hover: Color32::from_rgb(80, 80, 100),
            pattern_text: Color32::BLACK,

            playlist_bg: Color32::from_rgb(255, 255, 255),
            track_even: Color32::from_rgb(245, 245, 250),
            track_odd: Color32::from_rgb(235, 235, 240),
            clip_default: Color32::from_rgb(100, 150, 200),
            clip_pattern: Color32::from_rgb(150, 100, 200),
            clip_audio: Color32::from_rgb(100, 200, 150),

            text_primary: Color32::BLACK,
            text_secondary: Color32::from_rgb(80, 80, 80),
            border: Color32::from_rgb(180, 180, 200),
            accent: Color32::from_rgb(50, 100, 200),
            playhead: Color32::from_rgb(200, 50, 50),
        }
    }

    pub fn fl_studio() -> Self {
        Self {
            bg_primary: Color32::from_rgb(40, 40, 40),
            bg_secondary: Color32::from_rgb(50, 50, 50),
            bg_tertiary: Color32::from_rgb(60, 60, 60),

            panel_bg: Color32::from_rgb(45, 45, 45),
            panel_header: Color32::from_rgb(55, 55, 55),

            pattern_bg: Color32::from_rgb(65, 65, 65),
            pattern_bg_hover: Color32::from_rgb(75, 75, 75),
            pattern_handle: Color32::from_rgb(140, 140, 140),
            pattern_handle_hover: Color32::from_rgb(180, 180, 180),
            pattern_text: Color32::from_rgb(220, 220, 220),

            playlist_bg: Color32::from_rgb(35, 35, 35),
            track_even: Color32::from_rgb(45, 45, 45),
            track_odd: Color32::from_rgb(50, 50, 50),
            clip_default: Color32::from_rgb(80, 120, 180),
            clip_pattern: Color32::from_rgb(180, 80, 120),
            clip_audio: Color32::from_rgb(80, 180, 120),

            text_primary: Color32::from_rgb(220, 220, 220),
            text_secondary: Color32::from_rgb(160, 160, 160),
            border: Color32::from_rgb(80, 80, 80),
            accent: Color32::from_rgb(255, 130, 0), // FL Studio orange
            playhead: Color32::from_rgb(255, 130, 0),
        }
    }
}