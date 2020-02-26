import {memory} from "emulator_engine/emulator_engine_bg";
import {EmulatorEngine} from "emulator_engine";

const engine = EmulatorEngine.new();
const ram = new Uint8Array(memory.buffer, engine.ram(), engine.ram_size());
const rom = new Uint8Array(memory.buffer, engine.rom(), engine.rom_size());

export {
    engine, ram, rom
}