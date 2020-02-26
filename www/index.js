import {memory} from "pic16f8x_emu/pic16f8x_emu_bg";
import {EmulationEngine} from "pic16f8x_emu";

const engine = EmulationEngine.new();
const ram = new Uint8Array(memory.buffer, engine.ram(), engine.ram_size());
const rom = new Uint8Array(memory.buffer, engine.rom(), engine.rom_size());

// Bootstrap dummy code
import "./scss/app.scss"