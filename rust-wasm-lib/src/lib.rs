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
    moved_figure: IFigure,
    square_no: number,
    captured_figure_no?: number,
    captured_figure?: IFigure,
}
"#;

#[wasm_bindgen(skip_typescript)]
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct Move {
    moved_figure_no: i32,
    moved_figure: IFigure,
    square_no: i32,
    captured_figure_no: Option<i32>,
    captured_figure: Option<IFigure>,
}

impl fmt::Debug for IFigure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color: {}, Kind: {}", self.color, self.kind)
    }
}
impl fmt::Display for IFigure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color: {}, Kind: {}", self.color, self.kind)
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(moved_figure_no:{}, moved_figure:{}, sqare_no:{}, captured_figure_no:{:?})",
            self.moved_figure_no, self.moved_figure, self.square_no, self.captured_figure_no
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
                    self,
                    target_sqare_no,
                    captured_figure,
                    poss_moves,
                    figure_map,
                );
            }
            None => poss_moves.push(Move {
                moved_figure_no: *self.get_figure_no(),
                moved_figure: (*self.get_figure()).clone(),
                square_no: target_sqare_no,
                captured_figure_no: None,
                captured_figure: None,
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
        } else if diff % 11 == 0 {
            Direction::LeftUp
        } else {
            Direction::RightUp
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
            if figure_map.get(&poss_block_figure_no).is_none() {
                poss_moves.push(Move {
                    moved_figure_no: *self.get_figure_no(),
                    moved_figure: (*self.get_figure()).clone(),
                    square_no: poss_block_figure_no,
                    captured_figure_no: Some(captured_figure_no),
                    captured_figure: Some((*captured_figure).clone()),
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
                Self::add_poss_move_forward(self, target_sqare_no, &mut poss_moves, figure_map);
                let is_capture = match poss_moves.last() {
                    Some(mov) => mov.captured_figure_no.is_some(),
                    None => false,
                };
                if poss_moves.len() == len_before || is_capture {
                    break;
                }
            }
        }
        poss_moves
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
        vec![right_up, right_down, left_up, left_down]
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
        moves
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

        poss_moves
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
        vec![target_sqares_forward, target_sqares_backward]
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
                self,
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
    let poss_moves: Vec<Move> = if moved_figure.kind == "man" {
        let man = Man::new(moved_figure_no, (*moved_figure).clone());
        man.get_poss_moves(figure_map)
    } else {
        let king = King::new(moved_figure_no, (*moved_figure).clone());
        king.get_poss_moves(figure_map)
    };
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
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
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
    let mut figure_map: HashMap<i32, IFigure> = serde_wasm_bindgen::from_value(figure_map)?;
    let board: Board = Board::new(&mut figure_map);
    let result = board.get_winner();
    Ok(serde_wasm_bindgen::to_value(&result)?)
}

#[wasm_bindgen(typescript_custom_section)]
const possible_moves: &'static str = r#"
export function forced_moves(color: Color, figure_map: Map<number, IFigure>): Move[];
"#;

#[wasm_bindgen(skip_typescript)]
pub fn forced_moves(color: Color, figure_map: JsValue) -> Result<JsValue, JsError> {
    let mut figure_map: HashMap<i32, IFigure> = serde_wasm_bindgen::from_value(figure_map)?;
    let mut board: Board = Board::new(&mut figure_map);
    let forced_moves = board.get_forced_moves(&color);
    let mut first_forced_moves: Vec<Move> = vec![];
    for mov in &forced_moves {
        first_forced_moves.push(mov[0].clone());
    }
    Ok(serde_wasm_bindgen::to_value(&first_forced_moves)?)
}

#[wasm_bindgen(typescript_custom_section)]
const possible_moves: &'static str = r#"
export function get_best_move(color: Color, figure_map: Map<number, IFigure>): Move[];
"#;

#[wasm_bindgen(skip_typescript)]
pub fn get_best_move(color: Color, figure_map: JsValue) -> Result<JsValue, JsError> {
    let mut figure_map: HashMap<i32, IFigure> = serde_wasm_bindgen::from_value(figure_map)?;
    let start = instant::Instant::now();
    let mut board: Board = Board::new(&mut figure_map);
    let (bestval, mov) = board.minimax(10, i32::MIN, i32::MAX, color);
    let elapsed = start.elapsed();
    console::log_1(&format!("Elapsed: {elapsed:?}").into());
    console::log_1(&format!("Best val: {bestval:?}").into());
    Ok(serde_wasm_bindgen::to_value(&mov)?)
}

struct Board<'a> {
    figure_map: &'a mut HashMap<i32, IFigure>,
}

impl Board<'_> {
    fn new(figure_map: &mut HashMap<i32, IFigure>) -> Board {
        Board { figure_map }
    }

    fn make_move(&mut self, mov: &Move) {
        //Making the move
        self.figure_map.remove(&mov.moved_figure_no);
        self.figure_map
            .insert(mov.square_no, mov.moved_figure.clone());
        if let Some(captured_figure_no) = mov.captured_figure_no {
            self.figure_map.remove(&captured_figure_no);
        }
    }

    fn make_moves(&mut self, moves: &Vec<Move>) {
        for mov in moves {
            self.make_move(mov);
        }
    }

    fn unmake_move(&mut self, mov: &Move) {
        //Unmaking the move;
        self.figure_map
            .insert(mov.moved_figure_no, mov.moved_figure.clone());
        self.figure_map.remove(&mov.square_no);
        if let Some(captured_figure_no) = mov.captured_figure_no {
            if let Some(captured_figure) = &mov.captured_figure {
                self.figure_map
                    .insert(captured_figure_no, (*captured_figure).clone());
            }
        }
    }

    fn unmake_moves(&mut self, moves: &Vec<Move>) {
        for mov in moves {
            self.unmake_move(mov);
        }
    }

    fn get_forced_moves(&mut self, color: &Color) -> Vec<Vec<Move>> {
        //Get moves for color
        let figures = self.figure_map.iter().filter(|&(_, figure)| {
            figure.color
                == match color {
                    Color::White => "white",
                    Color::Black => "black",
                }
        });
        let mut capture_moves: Vec<Move> = vec![];
        for (figure_no, figure) in figures {
            let moves: Vec<Move> = get_poss_moves(*figure_no, figure, self.figure_map)
                .into_iter()
                .filter(|mov| mov.captured_figure_no.is_some())
                .collect();
            for mov in moves {
                capture_moves.push(mov);
            }
        }
        let (_depth, mut moves_vector) = self.get_forced_from_captures(&capture_moves);
        for moves in &mut moves_vector {
            moves.reverse();
        }
        moves_vector
    }

    fn get_forced_from_captures(&mut self, capture_moves: &Vec<Move>) -> (i32, Vec<Vec<Move>>) {
        if capture_moves.is_empty() {
            return (0, vec![]);
        }

        let mut tree_depths: Vec<(i32, Vec<Move>)> = vec![];
        for mov in capture_moves {
            self.make_move(mov);
            //Geting new moves, moved figure doesn't change
            let new_capture_moves: Vec<Move> =
                get_poss_moves(mov.square_no, &mov.moved_figure, self.figure_map)
                    .into_iter()
                    .filter(|mov| mov.captured_figure_no.is_some())
                    .collect();
            //Recursivly get depth and moves
            let (depth, moves) = self.get_forced_from_captures(&new_capture_moves);
            self.unmake_move(mov);
            let moves_size = moves.len();
            //Add current move
            let mut new_moves = moves;
            if moves_size == 0 {
                tree_depths.push((depth + 1, vec![(*mov).clone()]));
                continue;
            };
            for vec in &mut new_moves {
                vec.push((*mov).clone());
                tree_depths.push((depth + 1, vec.to_vec()));
            }
        }

        let max_depth = tree_depths
            .iter()
            .max_by_key(|(depth, _)| depth)
            .unwrap_or(&(i32::MIN, vec![]))
            .0;
        let mut max_depth_vectors: Vec<Vec<Move>> = vec![];
        let (_, moves_vectors): (Vec<i32>, Vec<Vec<Move>>) = tree_depths
            .into_iter()
            .filter(|(depth, _)| *depth == max_depth)
            .unzip();
        for moves in moves_vectors {
            max_depth_vectors.push(moves);
        }
        (max_depth, max_depth_vectors)
    }

    fn get_available_moves(&mut self, color: &Color) -> Vec<Vec<Move>> {
        let forced_moves = self.get_forced_moves(color);
        if forced_moves.iter().count() > 0 {
            return forced_moves;
        }
        let mut poss_moves: Vec<Vec<Move>> = vec![];
        let figures = self.figure_map.iter().filter(|&(_, figure)| {
            figure.color
                == match color {
                    Color::White => "white",
                    Color::Black => "black",
                }
        });
        for (moved_figure_no, moved_figure) in figures {
            for mov in get_poss_moves(*moved_figure_no, moved_figure, self.figure_map) {
                poss_moves.push(vec![mov]);
            }
        }
        poss_moves
    }

    fn get_winner(&self) -> Option<Color> {
        let mut result: Option<Color> = None;
        for color in ["black", "white"] {
            let any_poss_moves = self
                .figure_map
                .iter()
                .filter(|&(_, figure)| figure.color == color)
                .any(|(figure_no, figure)| {
                    let poss_moves = get_poss_moves(*figure_no, figure, self.figure_map);
                    !poss_moves.is_empty()
                });
            let any_figure = self
                .figure_map
                .iter()
                .any(|(_, figure)| figure.color == color);
            if !any_poss_moves || !any_figure {
                if color == "black" {
                    result = Some(Color::White);
                } else {
                    result = Some(Color::Black);
                };
            }
        }
        result
    }

    fn get_rating(&self) -> i32 {
        let mut color_rating: HashMap<&str, i32> =
            [("white", 0), ("black", 0)].iter().cloned().collect();
        for color in ["white", "black"] {
            for (_, figure) in self
                .figure_map
                .iter()
                .filter(|(_, figure)| figure.color == *color)
            {
                if figure.kind == "man" {
                    *color_rating.get_mut(color).unwrap() += 1;
                } else {
                    *color_rating.get_mut(color).unwrap() += 3;
                }
            }
        }
        color_rating["white"] - color_rating["black"]
    }

    fn minimax(
        &mut self,
        target_deph: i32,
        alpha: i32,
        beta: i32,
        color: Color,
    ) -> (i32, Vec<Move>) {
        if target_deph == 0 || self.get_winner().is_some() {
            return (self.get_rating(), vec![]);
        }

        let moves_vector = self.get_available_moves(&color);
        let mut best_move: Vec<Move> = vec![];
        if color == Color::White {
            let mut bestval = i32::MIN;
            let mut alpha = alpha;
            for mov in &moves_vector {
                self.make_moves(mov);
                let (value, _) = self.minimax(target_deph - 1, alpha, beta, Color::Black);
                self.unmake_moves(mov);
                if value > bestval {
                    bestval = value;
                    best_move = mov.to_vec();
                }
                alpha = cmp::max(alpha, bestval);
                if beta <= alpha {
                    break;
                }
            }
            (bestval, best_move)
        } else {
            let mut bestval = i32::MAX;
            let mut beta = beta;
            for mov in &moves_vector {
                self.make_moves(mov);
                let (value, _) = self.minimax(target_deph - 1, alpha, beta, Color::White);
                self.unmake_moves(mov);
                if value < bestval {
                    bestval = value;
                    best_move = mov.to_vec();
                }
                beta = cmp::min(beta, bestval);
                if beta <= alpha {
                    break;
                }
            }
            (bestval, best_move)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
