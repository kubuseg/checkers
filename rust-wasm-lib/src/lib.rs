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
pub fn possible_moves(i: i16, figure: JsValue, square_values: JsValue) -> Result<JsValue, JsValue> {
    let fig: IFigure = serde_wasm_bindgen::from_value(figure)?;
    let sqr_values: Vec<Option<IFigure>> = serde_wasm_bindgen::from_value(square_values)?;
    let mut poss_moves: Vec<i16> = vec![];
    if fig.color == "black" {
        if i%10 == 0 {
            poss_moves.push(i+11);
        } 
        else if i%10 == 9 {
            poss_moves.push(i+9);
        } 
        else {
            poss_moves.push(i+9);
            poss_moves.push(i+11);
        }
    } 
    else {
        if i%10 == 0 {
            poss_moves.push(i-9);
        } 
        else if i%10 == 9 {
            poss_moves.push(i-11);
        }
        else {
            poss_moves.push(i-9);
            poss_moves.push(i-11);
        }
    }
    Ok(serde_wasm_bindgen::to_value(&poss_moves)?)
}

// pub fn optional_new(_i: Option<ITextStyle>) -> TextStyle {
//     // parse JsValue
//     TextStyle::default()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
