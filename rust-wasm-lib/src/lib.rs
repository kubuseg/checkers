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

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct IFigure {
    color: String,
    kind: String,
}

#[wasm_bindgen]
pub fn possible_moves(i: u8, figure_map: JsValue) -> Result<JsValue, JsError> {
    let figure_map: HashMap<u8, IFigure> = serde_wasm_bindgen::from_value(figure_map)?;
    let fig: IFigure;
    match figure_map.get(&i) {
        Some(f) => fig = f.clone(),
        None => {
            fig = IFigure::default();
            return Err(JsError::new("Error loading fig"));
        }
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
                    Some(_) => (),
                    None => poss_moves.push(i + 11),
                }
                match figure_map.get(&(i + 9)) {
                    Some(_) => (),
                    None => poss_moves.push(i + 9),
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
                    Some(_) => (),
                    None => poss_moves.push(i - 11),
                }
                match figure_map.get(&(i - 9)) {
                    Some(_) => (),
                    None => poss_moves.push(i - 9),
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
