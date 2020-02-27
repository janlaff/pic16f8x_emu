<template>
  <div id="app">
    <button @click="runExample">Run example</button>
    <MemoryViewer :buffer="userMem"></MemoryViewer>
    <SFRViewer :bank="sfrBank"></SFRViewer>
  </div>
</template>

<script>
import MemoryViewer from "./components/MemoryViewer"
import SFRViewer from "./components/SFRViewer"
import {engine, readEngineMem} from "./emulator"
import {SFRBank} from "emulator_engine"

export default {
  name: 'App',
  components: { MemoryViewer, SFRViewer },
  data () {
    return {
      userMem: readEngineMem(engine.ram(), engine.ram_size()),
      sfrBank: SFRBank.new(),
    }
  },
  methods: {
    runExample () {
      engine.run_example()

      this.userMem = readEngineMem(engine.ram(), engine.ram_size())
      this.sfrBank = engine.read_sfrs()
    },
  }
}
</script>
