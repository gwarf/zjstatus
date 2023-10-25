use std::collections::BTreeMap;

use crate::render::FormattedPart;

const DEFAULT_CHAR: &str = "─";

#[derive(Default, PartialEq)]
pub enum BorderPosition {
    #[default]
    Top,
    Bottom,
}

pub struct BorderConfig {
    pub enabled: bool,
    pub char: String,
    pub format: FormattedPart,
    pub position: BorderPosition,
}

impl Default for BorderConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            char: DEFAULT_CHAR.to_string(),
            format: FormattedPart::default(),
            position: BorderPosition::default(),
        }
    }
}

impl BorderConfig {
    pub fn draw_if_enabled(&self, cols: usize) {
        if !self.enabled {
            return;
        }

        let output = self.char.repeat(cols);

        let mut newline = "";
        if self.position == BorderPosition::Top {
            newline = "\n";
        }

        print!("{}{}", self.format.format_string(output), newline);
    }
}

pub fn parse_border_config(config: BTreeMap<String, String>) -> Option<BorderConfig> {
    let enabled = match config.get("border_enabled") {
        Some(e) => matches!(e.as_str(), "true"),
        None => {
            return None;
        }
    };

    let char = match config.get("border_char") {
        Some(bc) => bc,
        None => DEFAULT_CHAR,
    };

    let format = match config.get("border_format") {
        Some(bfs) => bfs,
        None => "",
    };

    let position = match config.get("border_position") {
        Some(pos) => match pos.to_owned().as_str() {
            "bottom" => BorderPosition::Bottom,
            _ => BorderPosition::Top,
        },
        None => BorderPosition::Top,
    };

    Some(BorderConfig {
        enabled,
        char: char.to_string(),
        format: FormattedPart::from_format_string(format.to_string()),
        position,
    })
}
