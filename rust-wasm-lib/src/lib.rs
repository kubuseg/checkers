use serde::{Deserialize, Serialize};
use std::{collections::HashMap, vec};
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen(start)]
pub fn run() {
    init_panic_hook();
}

#[wasm_bindgen(typescript_custom_section)]
const IMove: &'static str = r#"
export interface IMove {
    squareNo: number;
    isCapture: boolean;
}
"#;

#[wasm_bindgen(skip_typescript)]
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Move {
    square_no: u8,
    is_capture: bool,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct IFigure {
    color: String,
    kind: String,
}

#[wasm_bindgen(typescript_custom_section)]
const possible_moves: &'static str = r#"
export function possible_moves(i: number, figure_map: Map<number, IFigure>): IMove[];
"#;

#[wasm_bindgen(skip_typescript)]
pub fn possible_moves(i: u8, figure_map: JsValue) -> Result<JsValue, JsError> {
    let figure_map: HashMap<u8, IFigure> = serde_wasm_bindgen::from_value(figure_map)?;
    let source_figure: IFigure;
    match figure_map.get(&i) {
        Some(f) => source_figure = f.clone(),
        None => source_figure = IFigure::default(),
    }

    let mut poss_moves: Vec<Move> = vec![];
    if source_figure.color == "black" {
        match i % 10 {
            0 => add_poss_move(i, i + 11, &source_figure, &mut poss_moves, &figure_map),
            9 => add_poss_move(i, i + 9, &source_figure, &mut poss_moves, &figure_map),
            _ => {
                add_poss_move(i, i + 11, &source_figure, &mut poss_moves, &figure_map);
                add_poss_move(i, i + 9, &source_figure, &mut poss_moves, &figure_map);
            }
        }
    } else {
        match i % 10 {
            0 => add_poss_move(i, i - 9, &source_figure, &mut poss_moves, &figure_map),
            9 => add_poss_move(i, i - 11, &source_figure, &mut poss_moves, &figure_map),
            _ => {
                add_poss_move(i, i - 9, &source_figure, &mut poss_moves, &figure_map);
                add_poss_move(i, i - 11, &source_figure, &mut poss_moves, &figure_map);
            }
        }
    }
    Ok(serde_wasm_bindgen::to_value(&poss_moves)?)
}

fn add_poss_move(
    source_figure_no: u8,
    captured_figure_no: u8,
    source_figure: &IFigure,
    poss_moves: &mut Vec<Move>,
    figure_map: &HashMap<u8, IFigure>,
) {
    match figure_map.get(&source_figure_no) {
        Some(captured_figure) => try_capture(
            source_figure_no,
            captured_figure_no,
            source_figure,
            captured_figure,
            poss_moves,
            figure_map,
        ),
        None => poss_moves.push(Move {
            square_no: captured_figure_no,
            is_capture: false,
        }),
    }
}

fn try_capture(
    source_figure_no: u8,
    captured_figure_no: u8,
    source_figure: &IFigure,
    captured_figure: &IFigure,
    poss_moves: &mut Vec<Move>,
    figure_map: &HashMap<u8, IFigure>,
) {
    if source_figure.color != captured_figure.color {
        let poss_block_figure_no = source_figure_no + 2 * (captured_figure_no - source_figure_no);
        match figure_map.get(&poss_block_figure_no) {
            Some(_) => (),
            None => poss_moves.push(Move {
                square_no: poss_block_figure_no,
                is_capture: true,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
