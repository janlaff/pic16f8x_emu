import {memory} from "emulator_engine/emulator_engine_bg"
import {EmulatorEngine, SFRBank} from "emulator_engine"

const engine = EmulatorEngine.new()

function readEngineMem(bufferPtr, bufferSize) {
    return new Uint8Array(memory.buffer, bufferPtr, bufferSize)
}

export {
    engine, readEngineMem
}
