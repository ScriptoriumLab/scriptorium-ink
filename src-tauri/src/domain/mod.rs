use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderState {
    pub visible: bool,
    pub x: f64,
    pub y: f64,
    pub candidates: Vec<String>,
    pub highlight_index: usize,
    pub page_index: usize,
    pub total_pages: usize,
}

impl Default for RenderState {
    fn default() -> Self {
        Self {
            visible: false,
            x: 0.0,
            y: 0.0,
            candidates: vec![],
            highlight_index: 0,
            page_index: 0,
            total_pages: 0,
        }
    }
}