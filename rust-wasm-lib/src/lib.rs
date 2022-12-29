use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(typescript_type = "IFigure")]
//     pub type IFigure;
// }

#[derive(Default, Serialize, Deserialize)]
pub struct IFigure {
    color: String,
    kind: String,
}


#[wasm_bindgen]
pub fn possible_moves(i: i32, figure: JsValue, square_values: JsValue) -> Result<JsValue, JsValue> {
    let fig: IFigure = serde_wasm_bindgen::from_value(figure)?;
    let sqr_values: Vec<Option<IFigure>> = serde_wasm_bindgen::from_value(square_values)?;
    let poss_moves: Vec<i32>;
    if fig.color == "black" {
        match i%10 {
            0 => poss_moves = vec![i+11],
            9 => poss_moves = vec![i+9],
            _ => poss_moves = vec![i+9, i+11],
        }
    } else {
        match i%10 {
            0 => poss_moves = vec![i-9],
            9 => poss_moves = vec![i-11],
            _ => poss_moves = vec![i-9, i-11]
        }
    }
    Ok(serde_wasm_bindgen::to_value(&poss_moves)?)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
