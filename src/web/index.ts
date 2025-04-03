import init, { greet, greet2, content } from './tsrust_lib.js';

window.greet = greet; // greet関数をグローバルに公開
window.greet2 = greet2; // greet2関数をグローバルに公開

async function run() {
  await init(); // Wasmモジュールの初期化
  // Rustで書いた関数を呼び出し
  console.info("< greet1 >");
  console.log(greet('World'));
  console.info("< greet2 >");
  greet2("world");
  console.info("< content >");
  content();
}

run();


console.log("hello world from TypeScript!")