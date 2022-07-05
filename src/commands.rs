//! Commands to select and manipulate parts of the puzzle.

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::puzzle::{
    traits::*, Face, LayerMask, PieceType, PuzzleTypeEnum, Twist, TwistDirection, TwistSelection,
};

/// Minimum number of moves for a partial scramble.
pub const PARTIAL_SCRAMBLE_MOVE_COUNT_MIN: usize = 1;
/// Maximum number of moves for a partial scramble.
pub const PARTIAL_SCRAMBLE_MOVE_COUNT_MAX: usize = 20;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Command {
    // File menu
    Open,
    Save,
    SaveAs,
    Exit,

    // Edit menu
    Undo,
    Redo,
    Reset,

    // Scramble menu
    ScrambleN(usize),
    ScrambleFull,

    // Puzzle menu
    NewPuzzle(PuzzleTypeEnum),

    ToggleBlindfold,

    #[serde(other)]
    None,
}
impl Default for Command {
    fn default() -> Self {
        Self::None
    }
}
impl Command {
    pub(crate) fn get_puzzle_type(&self) -> PuzzleTypeEnum {
        match self {
            Command::NewPuzzle(puzzle_type) => *puzzle_type,
            _ => PuzzleTypeEnum::default(),
        }
    }

    pub(crate) fn short_description(&self) -> String {
        match self {
            Command::Open => "Open".to_owned(),
            Command::Save => "Save".to_owned(),
            Command::SaveAs => "Save As".to_owned(),
            Command::Exit => "Exit".to_owned(),

            Command::Undo => "Undo".to_owned(),
            Command::Redo => "Redo".to_owned(),
            Command::Reset => "Reset".to_owned(),

            Command::ScrambleN(n) => format!("Scramble {n}"),
            Command::ScrambleFull => "Scramble fully".to_owned(),

            Command::NewPuzzle(ty) => format!("New {}", ty.name()),

            Command::ToggleBlindfold => "BLD".to_owned(),

            Command::None => String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum PuzzleCommand {
    SelectAxis(String),
    SelectLayers(LayerMask),
    Twist {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        axis: Option<String>,
        direction: String,
        layers: LayerMask,
    },
    Recenter {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        axis: Option<String>,
    },

    #[serde(other)]
    None,
}
impl Default for PuzzleCommand {
    fn default() -> Self {
        Self::None
    }
}
impl PuzzleCommand {
    pub fn short_description(&self, ty: PuzzleTypeEnum) -> String {
        match self {
            PuzzleCommand::SelectAxis(axis_name) => axis_name.to_owned(),
            PuzzleCommand::SelectLayers(layers) => layers.digits(),
            PuzzleCommand::Twist {
                axis,
                direction,
                layers,
            } => {
                // TODO
                return format!("TODO describe twist");
                // if let Some(f) = face {
                //     match Twist::from_face_with_layers(*f, direction.name(), *layer_mask) {
                //         Ok(twist) => twist.to_string(),
                //         Err(e) => format!("<invalid twist: {e}>"),
                //     }
                // } else {
                //     let l = if layer_mask.is_default() {
                //         String::new()
                //     } else {
                //         layer_mask.short_description()
                //     };
                //     match face {
                //         Some(f) => format!("{l}{}{}", f.symbol(), direction.symbol()),
                //         None => format!("{l}Ø{}", direction.name()),
                //     }
                // }
            }
            PuzzleCommand::Recenter { axis } => match axis {
                Some(f) => {
                    // TODO
                    return format!("TODO describe recenter");
                    // match Twist::from_face_recenter(*f) {
                    //     Ok(twist) => twist.to_string(),
                    //     Err(e) => format!("<invalid twist: {e}>"),
                    // }
                }
                None => format!("Recenter"),
            },

            PuzzleCommand::None => String::new(),
        }
    }
}
