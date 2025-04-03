// main.rs

mod common; // common.rsで定義した内容を使用する

use serde_json;


#[cfg(not(feature = "web"))]
fn main() {
    // cargo run で実行した時に動く処理

    use js_sys::wasm_bindgen::JsValue;
    println!("Hello World in Native");

    //
    let content = common::Content {
        title: "Hello".to_string(),
        content: "World".to_string(),
    };

    println!("content: {:?}", content);

    // serdeによるJSON変換
    println!("content: {:?}", serde_json::to_string(&content).unwrap());
}



#[cfg(feature = "web")]
fn main() {
    // 使わない
}