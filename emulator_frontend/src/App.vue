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
      userMem: ['lol']
    }
  },
  mounted () {
    this.refreshRam()
  },
  methods: {
    runExample () {
      engine.run_example()
      this.refreshRam()
    },
    refreshRam () {
      const mem = readEngineMem(engine.ram(), engine.ram_size())
      this.userMem = mem
    }
  }
}
</script>
