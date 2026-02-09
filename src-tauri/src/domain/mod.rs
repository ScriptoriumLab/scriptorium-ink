use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderState {
    pub visible: bool,
    pub candidates: Vec<String>,
    pub highlight_index: usize,
    pub page_index: usize,
    pub total_pages: usize,
}

impl Default for RenderState {
    fn default() -> Self {
        Self {
            visible: false,
            candidates: vec![],
            highlight_index: 0,
            page_index: 0,
            total_pages: 0,
        }
    }
}