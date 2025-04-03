// lib.rs

#![cfg(feature = "web")]
// Javascript/TypeScriptから呼び出すための関数を定義する


mod jsapi; // jsapi.rsで定義した内容を使用する
mod common; // common.rsで定義した内容を使用する

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! from Rust 1", name)
}

#[wasm_bindgen]
pub fn greet2(name: &str) {
    console_log!("abc",0,1,2,[0,1,2],vec![0,1,2]);
    console_log!(format!("Hello, {}! from Rust 2", name));
}

#[wasm_bindgen]
pub fn content() {
    let content = common::Content {
        title: "Hello".to_string(),
        content: "World".to_string(),
    };

    console_log!(format!("content: {:?}", content));

    // serdeによるJSON変換
    console_log!(format!("content: {:?}", serde_json::to_string(&content).unwrap()));
}