use serde::{Deserialize, Serialize};
use std::cmp;
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
    square_no: i32,
    is_capture: bool,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct IFigure {
    color: String,
    kind: String,
}
enum Zone {
    LeftBorder,
    Middle,
    RightBorder,
}

#[wasm_bindgen(typescript_custom_section)]
const possible_moves: &'static str = r#"
export function possible_moves(clicked_sqare_no: number, figure_map: Map<number, IFigure>): Move[];
"#;

struct King {
    figure_no: i32,
    figure: IFigure,
    zone: Zone,
}

impl King {
    pub fn new(figure_no: i32, figure: IFigure) -> Self {
        let zone: Zone = match figure_no % 10 {
            0 => Zone::LeftBorder,
            9 => Zone::RightBorder,
            _ => Zone::Middle,
        };
        Self {
            figure_no,
            figure,
            zone,
        }
    }

    fn get_target_sqares_by_direction(&self, moves_no: i32, step: i32, moves_up: bool) -> Vec<i32> {
        let mut moves: Vec<i32> = vec![];
        if moves_up {
            for mov in 1..=moves_no {
                moves.push(self.figure_no - mov * step);
            }
        } else {
            for mov in 1..=moves_no {
                moves.push(self.figure_no + mov * step)
            }
        }
        return moves;
    }

    fn get_target_sqares(&self) -> [Vec<i32>; 4] {
        let right_up_moves = cmp::min(9 - self.figure_no % 10, self.figure_no / 10);
        let right_up = Self::get_target_sqares_by_direction(self, right_up_moves, 9, true);
        let left_up_moves = cmp::min(self.figure_no % 10, self.figure_no / 10);
        let left_up = Self::get_target_sqares_by_direction(self, left_up_moves, 11, true);
        let right_down_moves = cmp::min(9 - self.figure_no % 10, 9 - self.figure_no / 10);
        let right_down = Self::get_target_sqares_by_direction(self, right_down_moves, 11, false);
        let left_down_moves = cmp::min(self.figure_no % 10, 9 - self.figure_no / 10);
        let left_down = Self::get_target_sqares_by_direction(self, left_down_moves, 9, false);
        return [right_up, right_down, left_up, left_down];
    }

    fn get_poss_moves(&self, figure_map: &HashMap<i32, IFigure>) {
        let movesArray = Self::get_target_sqares(self);
        for moves in movesArray {
            for mov in moves {
                
            }
        }
    }
}

struct Man {
    figure_no: i32,
    figure: IFigure,
    zone: Zone,
}

impl Man {
    pub fn new(figure_no: i32, figure: IFigure) -> Self {
        let zone: Zone = match figure_no % 10 {
            0 => Zone::LeftBorder,
            9 => Zone::RightBorder,
            _ => Zone::Middle,
        };
        Self {
            figure_no,
            figure,
            zone,
        }
    }

    fn get_poss_moves(&self, figure_map: &HashMap<i32, IFigure>) -> Vec<Move> {
        let mut poss_moves: Vec<Move> = vec![];
        let (target_sqares_forward, target_sqares_backward) = Self::get_target_sqares(self);

        for target_sqare_no in &target_sqares_forward {
            Self::add_poss_move_forward(self, *target_sqare_no, &mut poss_moves, figure_map);
        }
        for target_sqare_no in &target_sqares_backward {
            Self::add_poss_move_backwards(self, *target_sqare_no, &mut poss_moves, figure_map);
        }

        return poss_moves;
    }

    fn get_target_sqares(&self) -> (Vec<i32>, Vec<i32>) {
        let mut target_sqares_forward: Vec<i32> = match self.zone {
            Zone::LeftBorder => vec![11],
            Zone::Middle => vec![9, 11],
            Zone::RightBorder => vec![9],
        };
        if self.figure.color == "white" {
            for sqare_no in &mut target_sqares_forward {
                *sqare_no += self.figure_no - 20;
            }
        } else {
            for sqare_no in &mut target_sqares_forward {
                *sqare_no += self.figure_no;
            }
        }
        let mut target_sqares_backward = target_sqares_forward.clone();
        if self.figure.color == "black" {
            for sqare_no in &mut target_sqares_backward {
                *sqare_no += 20;
            }
        } else {
            for sqare_no in &mut target_sqares_backward {
                *sqare_no -= 20;
            }
        }
        return (target_sqares_forward, target_sqares_backward);
    }

    fn add_poss_move_forward(
        &self,
        target_sqare_no: i32,
        poss_moves: &mut Vec<Move>,
        figure_map: &HashMap<i32, IFigure>,
    ) {
        match figure_map.get(&target_sqare_no) {
            Some(captured_figure) => {
                Self::try_capture(
                    &self,
                    target_sqare_no,
                    captured_figure,
                    poss_moves,
                    figure_map,
                );
            }
            None => poss_moves.push(Move {
                square_no: target_sqare_no,
                is_capture: false,
            }),
        }
    }

    fn add_poss_move_backwards(
        &self,
        captured_figure_no: i32,
        poss_moves: &mut Vec<Move>,
        figure_map: &HashMap<i32, IFigure>,
    ) {
        match figure_map.get(&captured_figure_no) {
            Some(captured_figure) => {
                Self::try_capture(
                    &self,
                    captured_figure_no,
                    captured_figure,
                    poss_moves,
                    figure_map,
                );
            }
            None => (),
        }
    }

    fn try_capture(
        &self,
        captured_figure_no: i32,
        captured_figure: &IFigure,
        poss_moves: &mut Vec<Move>,
        figure_map: &HashMap<i32, IFigure>,
    ) {
        //Check if captured figure is the enemy and it isn't by the border
        if self.figure.color != captured_figure.color
            && ![0, 9].contains(&(captured_figure_no % 10))
            && !(0..10).contains(&captured_figure_no)
            && !(90..100).contains(&captured_figure_no)
        {
            //Check for possible block
            let poss_block_figure_no = self.figure_no + 2 * (captured_figure_no - self.figure_no);
            match figure_map.get(&poss_block_figure_no) {
                Some(_) => (),
                None => poss_moves.push(Move {
                    square_no: poss_block_figure_no,
                    is_capture: true,
                }),
            }
        }
    }
}

#[wasm_bindgen(skip_typescript)]
pub fn possible_moves(moved_figure_no: i32, figure_map: JsValue) -> Result<JsValue, JsError> {
    let figure_map: HashMap<i32, IFigure> = serde_wasm_bindgen::from_value(figure_map)?;

    let moved_figure: IFigure;
    match figure_map.get(&moved_figure_no) {
        Some(figure) => moved_figure = figure.clone(),
        None => moved_figure = IFigure::default(),
    }

    let poss_moves: Vec<Move>;
    if moved_figure.kind == "man" {
        let man = Man::new(moved_figure_no, moved_figure);
        poss_moves = man.get_poss_moves(&figure_map);
    } else {
    }

    Ok(serde_wasm_bindgen::to_value(&poss_moves)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
