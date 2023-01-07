use core::fmt;
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
    moved_figure_no: number,
    square_no: number;
    is_capture: boolean;
    captured_figure_no?: number;
}
"#;

#[wasm_bindgen(skip_typescript)]
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct Move {
    moved_figure_no: i32,
    square_no: i32,
    is_capture: bool,
    captured_figure_no: Option<i32>,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(moved_no:{}, sqare_no:{}, is_cap:{})",
            self.moved_figure_no, self.square_no, self.is_capture
        )
    }
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

#[derive(Clone, Debug)]
enum Direction {
    RightUp = -9,
    RightDown = 11,
    LeftUp = -11,
    LeftDown = 9,
}

trait GetMoves {
    fn new(figure_no: i32, figure: IFigure) -> Self;

    fn get_figure(&self) -> &IFigure;
    fn get_figure_no(&self) -> &i32;

    fn get_target_sqares(&self) -> Vec<Vec<i32>>;
    fn get_poss_moves(&self, figure_map: &HashMap<i32, IFigure>) -> Vec<Move>;

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
                moved_figure_no: *self.get_figure_no(),
                square_no: target_sqare_no,
                is_capture: false,
                captured_figure_no: None,
            }),
        }
    }

    fn get_capture_direction(&self, captured_figure_no: i32) -> Direction {
        let diff = captured_figure_no - self.get_figure_no();
        if diff > 0 {
            if diff % 11 == 0 {
                Direction::RightDown
            } else {
                Direction::LeftDown
            }
        } else {
            if diff % 11 == 0 {
                Direction::LeftUp
            } else {
                Direction::RightUp
            }
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
        if self.get_figure().color != captured_figure.color
            && ![0, 9].contains(&(captured_figure_no % 10)) //check if isn't at border
            && !(0..10).contains(&captured_figure_no) //check board top
            && !(90..100).contains(&captured_figure_no)
        //check board floor
        {
            //Check for possible block
            let poss_block_figure_no =
                captured_figure_no + self.get_capture_direction(captured_figure_no) as i32;
            if let None = figure_map.get(&poss_block_figure_no) {
                poss_moves.push(Move {
                    moved_figure_no: *self.get_figure_no(),
                    square_no: poss_block_figure_no,
                    is_capture: true,
                    captured_figure_no: Some(captured_figure_no),
                });
            }
        }
    }
}

struct King {
    figure_no: i32,
    figure: IFigure,
}

impl GetMoves for King {
    fn new(figure_no: i32, figure: IFigure) -> Self {
        Self { figure_no, figure }
    }
    fn get_figure(&self) -> &IFigure {
        &self.figure
    }
    fn get_figure_no(&self) -> &i32 {
        &self.figure_no
    }

    fn get_poss_moves(&self, figure_map: &HashMap<i32, IFigure>) -> Vec<Move> {
        let mut poss_moves: Vec<Move> = vec![];
        let moves_array = Self::get_target_sqares(self);
        for moves in moves_array {
            for target_sqare_no in moves {
                let len_before = poss_moves.len();
                Self::add_poss_move_forward(&self, target_sqare_no, &mut poss_moves, figure_map);
                let is_capture = match poss_moves.last() {
                    Some(figure) => figure.is_capture == true,
                    None => false,
                };
                if poss_moves.len() == len_before || is_capture {
                    break;
                }
            }
        }
        return poss_moves;
    }

    fn get_target_sqares(&self) -> Vec<Vec<i32>> {
        let right_up_moves = cmp::min(9 - self.figure_no % 10, self.figure_no / 10);
        let right_up = Self::get_target_sqares_by_direction(self, right_up_moves, 9, true);
        let left_up_moves = cmp::min(self.figure_no % 10, self.figure_no / 10);
        let left_up = Self::get_target_sqares_by_direction(self, left_up_moves, 11, true);
        let right_down_moves = cmp::min(9 - self.figure_no % 10, 9 - self.figure_no / 10);
        let right_down = Self::get_target_sqares_by_direction(self, right_down_moves, 11, false);
        let left_down_moves = cmp::min(self.figure_no % 10, 9 - self.figure_no / 10);
        let left_down = Self::get_target_sqares_by_direction(self, left_down_moves, 9, false);
        return vec![right_up, right_down, left_up, left_down];
    }
}

impl King {
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
}

struct Man {
    figure_no: i32,
    figure: IFigure,
    zone: Zone,
}

impl GetMoves for Man {
    fn new(figure_no: i32, figure: IFigure) -> Self {
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
    fn get_figure(&self) -> &IFigure {
        &self.figure
    }
    fn get_figure_no(&self) -> &i32 {
        &self.figure_no
    }

    fn get_poss_moves(&self, figure_map: &HashMap<i32, IFigure>) -> Vec<Move> {
        let mut poss_moves: Vec<Move> = vec![];
        let all_moves = Self::get_target_sqares(self);
        let target_sqares_forward = &all_moves[0];
        let target_sqares_backward = &all_moves[1];

        for target_sqare_no in target_sqares_forward {
            Self::add_poss_move_forward(self, *target_sqare_no, &mut poss_moves, figure_map);
        }
        for target_sqare_no in target_sqares_backward {
            Self::add_poss_move_backwards(self, *target_sqare_no, &mut poss_moves, figure_map);
        }

        return poss_moves;
    }

    fn get_target_sqares(&self) -> Vec<Vec<i32>> {
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
        if self.figure.color == "white" {
            for sqare_no in &mut target_sqares_backward {
                *sqare_no += 20;
            }
        } else {
            for sqare_no in &mut target_sqares_backward {
                *sqare_no -= 20;
            }
        }
        return vec![target_sqares_forward, target_sqares_backward];
    }
}

impl Man {
    fn add_poss_move_backwards(
        &self,
        captured_figure_no: i32,
        poss_moves: &mut Vec<Move>,
        figure_map: &HashMap<i32, IFigure>,
    ) {
        if let Some(captured_figure) = figure_map.get(&captured_figure_no) {
            Self::try_capture(
                &self,
                captured_figure_no,
                captured_figure,
                poss_moves,
                figure_map,
            );
        }
    }
}

fn get_poss_moves(
    moved_figure_no: i32,
    moved_figure: &IFigure,
    figure_map: &HashMap<i32, IFigure>,
) -> Vec<Move> {
    let poss_moves: Vec<Move>;
    if moved_figure.kind == "man" {
        let man = Man::new(moved_figure_no, (*moved_figure).clone());
        poss_moves = man.get_poss_moves(&figure_map);
    } else {
        let king = King::new(moved_figure_no, (*moved_figure).clone());
        poss_moves = king.get_poss_moves(&figure_map);
    }
    poss_moves
}

#[wasm_bindgen(typescript_custom_section)]
const possible_moves: &'static str = r#"
export function possible_moves(clicked_sqare_no: number, figure_map: Map<number, IFigure>): Move[];
"#;

#[wasm_bindgen(skip_typescript)]
pub fn possible_moves(moved_figure_no: i32, figure_map: JsValue) -> Result<JsValue, JsError> {
    let figure_map: HashMap<i32, IFigure> = serde_wasm_bindgen::from_value(figure_map)?;

    let moved_figure: IFigure;
    if let Some(figure) = figure_map.get(&moved_figure_no) {
        moved_figure = figure.clone();
    } else {
        moved_figure = IFigure::default()
    }

    let poss_moves = get_poss_moves(moved_figure_no, &moved_figure, &figure_map);

    Ok(serde_wasm_bindgen::to_value(&poss_moves)?)
}

#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Color {
    Black,
    White,
}

#[wasm_bindgen(typescript_custom_section)]
const possible_moves: &'static str = r#"
export function get_winner(figure_map: Map<number, IFigure>): Color?;
"#;

#[wasm_bindgen(skip_typescript)]
pub fn get_winner(figure_map: JsValue) -> Result<JsValue, JsError> {
    let figure_map: HashMap<i32, IFigure> = serde_wasm_bindgen::from_value(figure_map)?;
    let mut result: Option<Color> = None;
    for color in vec!["black", "white"] {
        let any_poss_moves = figure_map
            .iter()
            .filter(|&(_, figure)| figure.color == color)
            .any(|(figure_no, figure)| {
                let poss_moves = get_poss_moves(*figure_no, figure, &figure_map);
                !poss_moves.is_empty()
            });
        let any_figure = figure_map.iter().any(|(_, figure)| figure.color == color);
        if !any_poss_moves || !any_figure {
            if color == "black" {
                result = Some(Color::White);
            } else {
                result = Some(Color::Black);
            };
        }
    }
    Ok(serde_wasm_bindgen::to_value(&result)?)
}

#[wasm_bindgen(typescript_custom_section)]
const possible_moves: &'static str = r#"
export function forced_moves(color: Color, figure_map: Map<number, IFigure>): Move[];
"#;

#[wasm_bindgen(skip_typescript)]
pub fn forced_moves(color: Color, figure_map: JsValue) -> Result<JsValue, JsError> {
    let figure_map: HashMap<i32, IFigure> = serde_wasm_bindgen::from_value(figure_map)?;

    //Get moves for color
    let figures = figure_map.iter().filter(|&(_, figure)| {
        figure.color
            == match color {
                Color::White => "white",
                Color::Black => "black",
            }
    });
    let mut capture_moves: Vec<Move> = vec![];
    for (figure_no, figure) in figures {
        let moves: Vec<Move> = get_poss_moves(*figure_no, figure, &figure_map)
            .into_iter()
            .filter(|mov| mov.is_capture)
            .collect();
        for mov in moves {
            capture_moves.push(mov);
        }
    }
    let (_depth, mut moves) = get_forced_moves(&capture_moves, &figure_map);
    //Reverse because forced moves returns moves in backward order
    moves.reverse();
    Ok(serde_wasm_bindgen::to_value(&moves)?)
}

fn get_forced_moves(
    capture_moves: &Vec<Move>,
    figure_map: &HashMap<i32, IFigure>,
) -> (i32, Vec<Move>) {
    if capture_moves.is_empty() {
        return (0, vec![]);
    }

    let mut tree_depths: Vec<(i32, Move)> = vec![];
    for mov in capture_moves {
        let mut new_figure_map = figure_map.clone();

        if let Some(moved_figure) = figure_map.get(&mov.moved_figure_no) {
            //Making the move
            new_figure_map.remove(&mov.moved_figure_no);
            new_figure_map.insert(mov.square_no, (*moved_figure).clone());
            if let Some(captured_figure_no) = mov.captured_figure_no {
                new_figure_map.remove(&captured_figure_no);
            }
            //Geting new moves, moved figure doesn't change
            let new_capture_moves: Vec<Move> =
                get_poss_moves(mov.square_no, moved_figure, &new_figure_map)
                    .into_iter()
                    .filter(|mov| mov.is_capture)
                    .collect();
            //Recursivly get depth and moves
            let (depth, _) = get_forced_moves(&new_capture_moves, &new_figure_map);
            //Add current move
            tree_depths.push((depth, (*mov).clone()));
        }
    }

    let max_depth = tree_depths
        .iter()
        .max_by_key(|(depth, _)| depth)
        .unwrap_or(&(i32::MIN, Move::default()))
        .0;
    let mut max_depth_moves:Vec<Move> = vec![];
    for (_, mov) in tree_depths.iter().filter(|(depth, _)| *depth == max_depth) {
        max_depth_moves.push((*mov).clone());
    }
    return (max_depth + 1, max_depth_moves);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
