import init, { start_gui, greet, greet2, content } from './tsrust_lib.js';

async function run() {
    await init();
    // Rustで書いた関数を呼び出し
    console.info("< greet1 >");
    console.log(greet('World'));
    console.info("< greet2 >");
    greet2("world");
    console.info("< content >");
    content();
    // eguiによるGUIを起動
    start_gui();
}

run();