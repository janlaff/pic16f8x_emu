import init, { run_app } from './pkg/pic16f8x_emu.js';
async function main() {
   await init('/pkg/pic16f8x_emu_bg.wasm');
   run_app();
}
main()
