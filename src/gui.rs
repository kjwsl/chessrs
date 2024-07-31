use egui;

pub struct ChessProgram {
    name: String,
    size: (f32, f32),
}

impl Default for ChessProgram {
    fn default() -> Self {
        Self {
            name: "Chess Program".to_string(),
            size: (800.0, 600.0),
        }
    }
}
