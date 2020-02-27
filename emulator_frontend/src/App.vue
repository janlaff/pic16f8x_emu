<template>
  <v-app>
    <Controls @play="runExample"></Controls>
    <SFRViewer :bank="sfrBank"></SFRViewer>
  </v-app>
</template>

<script>
import MemoryViewer from "./components/MemoryViewer"
import SFRViewer from "./components/SFRViewer"
import {engine, readEngineMem} from "./emulator"
import {SFRBank} from "emulator_engine"
import Controls from "./components/Controls"

export default {
  name: 'App',
  components: { Controls, MemoryViewer, SFRViewer },
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
