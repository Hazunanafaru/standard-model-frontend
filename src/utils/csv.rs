use crate::content::Author;
use gloo::console::log;
use std::{self, error::Error};
use wasm_bindgen::JsValue;

pub fn read_author_csv() -> Result<Author, Box<dyn Error>> {
    let path = "/home/husni.zuhdi@accelbyte.net/Documents/Codes/personal/standard-model-frontend/data/author.csv";
    let mut reader = csv::Reader::from_path(path)?;
    let mut iter = reader.deserialize();

    if let Some(result) = iter.next() {
        let record: Author = result?;
        log!(JsValue::from(&record.name));
        Ok(record)
    } else {
        Err(From::from("expected at least one author, but got none :("))
    }
}
