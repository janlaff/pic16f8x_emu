import init, { run_app } from './pic16f8x_emu.js';
async function main() {
   await init('./pic16f8x_emu_bg.wasm');
   run_app();
}
main()
