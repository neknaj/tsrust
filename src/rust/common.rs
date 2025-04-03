// common.rs

// NativeからもWebからも使う共通の処理を定義する場所
// ここに書いたものは、NativeでもWebでも使える

use serde::{Serialize, Deserialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export, export_to = "../src/web/model.ts")]
pub struct Content {
    pub title: String,
    pub content: String,
}