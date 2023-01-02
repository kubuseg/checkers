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
const Move: &'static str = r#"
export interface Move {
    square_no: number;
    is_capture: boolean;
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
export function possible_moves(clicked_sqare_no: number, figure_map: Map<number, IFigure>): IMove[];
"#;

#[wasm_bindgen(skip_typescript)]
pub fn possible_moves(clicked_sqare_no: u8, figure_map: JsValue) -> Result<JsValue, JsError> {
    let figure_map: HashMap<u8, IFigure> = serde_wasm_bindgen::from_value(figure_map)?;

    let moved_figure: IFigure;
    match figure_map.get(&clicked_sqare_no) {
        Some(figure) => moved_figure = figure.clone(),
        None => moved_figure = IFigure::default(),
    }

    let mut poss_moves: Vec<Move> = vec![];
    if moved_figure.color == "black" {
        match clicked_sqare_no % 10 {
            0 => try_add_poss_move(
                clicked_sqare_no,
                clicked_sqare_no + 11,
                &moved_figure,
                &mut poss_moves,
                &figure_map,
            ),
            9 => try_add_poss_move(
                clicked_sqare_no,
                clicked_sqare_no + 9,
                &moved_figure,
                &mut poss_moves,
                &figure_map,
            ),
            _ => {
                try_add_poss_move(
                    clicked_sqare_no,
                    clicked_sqare_no + 11,
                    &moved_figure,
                    &mut poss_moves,
                    &figure_map,
                );
                try_add_poss_move(
                    clicked_sqare_no,
                    clicked_sqare_no + 9,
                    &moved_figure,
                    &mut poss_moves,
                    &figure_map,
                );
            }
        }
    } else {
        match clicked_sqare_no % 10 {
            0 => try_add_poss_move(
                clicked_sqare_no,
                clicked_sqare_no - 9,
                &moved_figure,
                &mut poss_moves,
                &figure_map,
            ),
            9 => try_add_poss_move(
                clicked_sqare_no,
                clicked_sqare_no - 11,
                &moved_figure,
                &mut poss_moves,
                &figure_map,
            ),
            _ => {
                try_add_poss_move(
                    clicked_sqare_no,
                    clicked_sqare_no - 9,
                    &moved_figure,
                    &mut poss_moves,
                    &figure_map,
                );
                try_add_poss_move(
                    clicked_sqare_no,
                    clicked_sqare_no - 11,
                    &moved_figure,
                    &mut poss_moves,
                    &figure_map,
                );
            }
        }
    }
    Ok(serde_wasm_bindgen::to_value(&poss_moves)?)
}

fn try_add_poss_move(
    moved_figure_no: u8,
    captured_figure_no: u8,
    moved_figure: &IFigure,
    poss_moves: &mut Vec<Move>,
    figure_map: &HashMap<u8, IFigure>,
) {
    match figure_map.get(&captured_figure_no) {
        Some(captured_figure) => {
            try_capture(
                moved_figure_no,
                captured_figure_no,
                moved_figure,
                captured_figure,
                poss_moves,
                figure_map,
            );
        }
        None => poss_moves.push(Move {
            square_no: captured_figure_no,
            is_capture: false,
        }),
    }
}

fn try_capture(
    moved_figure_no: u8,
    captured_figure_no: u8,
    moved_figure: &IFigure,
    captured_figure: &IFigure,
    poss_moves: &mut Vec<Move>,
    figure_map: &HashMap<u8, IFigure>,
) {
    //Check if captured figure is the enemy and it isn't by the border
    if moved_figure.color != captured_figure.color && !vec![0, 9].contains(&(captured_figure_no % 10)) {
        let poss_block_figure_no = moved_figure_no + 2 * (captured_figure_no - moved_figure_no);
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
