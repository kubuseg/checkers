use serde::{Deserialize, Serialize};
use std::{collections::HashMap, vec};
use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(typescript_type = "IFigure")]
//     pub type IFigure;
// }

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct IFigure {
    color: String,
    kind: String,
}

#[wasm_bindgen]
pub fn possible_moves(i: u8, figure_map: JsValue) -> Result<JsValue, JsError> {
    let figure_map: HashMap<u8, IFigure> = serde_wasm_bindgen::from_value(figure_map)?;
    let fig:IFigure;
    match figure_map.get(&i) {
        Some(f) => fig = f.clone(),
        None => {fig = IFigure::default(); return Err(JsError::new("Error loading fig"))}
    }

    let mut poss_moves: Vec<u8> = vec![];
    if fig.color == "black" {
        match i % 10 {
            0 => match figure_map.get(&(i + 11)) {
                Some(_) => (),
                None => poss_moves = vec![i + 11],
            },
            9 => match figure_map.get(&(i + 9)) {
                Some(_) => (),
                None => poss_moves = vec![i + 9],
            },
            _ => {
                match figure_map.get(&(i + 11)) {
                    Some(_) => poss_moves.push(i + 11),
                    None => (),
                }
                match figure_map.get(&(i + 9)) {
                    Some(_) => poss_moves.push(i + 9),
                    None => (),
                }
            }
        }
    } else {
        match i % 10 {
            0 => match figure_map.get(&(i - 9)) {
                Some(_) => (),
                None => poss_moves = vec![i - 9],
            },
            9 => match figure_map.get(&(i - 11)) {
                Some(_) => (),
                None => poss_moves = vec![i - 11],
            },
            _ => {
                match figure_map.get(&(i - 11)) {
                    Some(_) => poss_moves.push(i - 11),
                    None => (),
                }
                match figure_map.get(&(i - 9)) {
                    Some(_) => poss_moves.push(i - 9),
                    None => (),
                }
            }
        }
    }
    Ok(serde_wasm_bindgen::to_value(&poss_moves)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
