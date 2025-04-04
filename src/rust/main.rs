// main.rs

mod common; // common.rsで定義した内容を使用する
mod gui;

#[cfg(not(feature = "web"))]
fn main() {
    // cargo run で実行した時に動く処理

    println!("Hello World in Native");

    //
    let content = common::Content {
        title: "Hello".to_string(),
        content: "World".to_string(),
    };

    println!("content: {:?}", content);

    // serdeによるJSON変換
    println!("content: {:?}", serde_json::to_string(&content).unwrap());

    // gui
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|_cc| Ok(Box::new(gui::MyApp::default())))
    ).ok();
}

#[cfg(feature = "web")]
fn main() {
    // Not used in web builds
}