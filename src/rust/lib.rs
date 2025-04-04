// lib.rs

#![cfg(feature = "web")]
#![cfg(target_arch = "wasm32")]
// Javascript/TypeScriptから呼び出すための関数を定義する

mod jsapi; // jsapi.rsで定義した内容を使用する
mod common; // common.rsで定義した内容を使用する
mod gui; // gui.rsで定義した内容を使用する

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

    // Debugによる表示
    console_log!(format!("content: {:?}", content));

    // serdeによるJSON変換
    console_log!(format!("content: {:?}", serde_json::to_string(&content).unwrap()));

    // javascriptのconsole.logと同じような表示
    console_log!(&content);
}

#[wasm_bindgen]
pub fn start_gui() -> Result<(), JsValue> {
    use wasm_bindgen::JsCast;
    use web_sys::HtmlCanvasElement;

    // Redirect panic messages to the browser console
    console_error_panic_hook::set_once();

    // Get the canvas element and convert it to the correct type
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("screen")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    wasm_bindgen_futures::spawn_local(async move {
        let web_options = eframe::WebOptions::default();
        eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(gui::MyApp::default()))),
            )
            .await
            .expect("failed to start eframe");
    });

    Ok(())
}