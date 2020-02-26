<template>
  <div id="app">
    <button @click="runExample">Run example</button>
    <MemoryViewer :buffer="userMem"></MemoryViewer>
  </div>
</template>

<script>
import MemoryViewer from "./components/MemoryViewer"
import {engine, readEngineMem} from "./emulator"

export default {
  name: 'App',
  components: { MemoryViewer },
  data () {
    return {
      userMem: readEngineMem(engine.ram(), engine.ram_size())
    }
  },
  methods: {
    runExample () {
      engine.run_example()

      this.userMem = readEngineMem(engine.ram(), engine.ram_size())

      const sfr_bank = engine.read_sfrs()
      console.log(sfr_bank)
    },
  }
}
</script>
