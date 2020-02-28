<template>
  <v-app style="height: 100vh; overflow-y: hidden">
    <div class="d-flex" style="height: 100%">
      <DebugViewer></DebugViewer>
      <SFRViewer :bank="sfrBank"></SFRViewer>
    </div>
  </v-app>
</template>

<script>
import DebugViewer from "./components/DebugViewer";
import SFRViewer from "./components/SFRViewer";
import { engine, readEngineMem } from "./emulator";
import { SFRBank } from "emulator_engine";
import Controls from "./components/Controls";

export default {
  name: "App",
  components: { Controls, DebugViewer, SFRViewer },
  data() {
    return {
      userMem: readEngineMem(engine.ram(), engine.ram_size()),
      sfrBank: SFRBank.new()
    };
  },
  mounted () {
    this.$root.$on('step', this.play)
    this.$root.$on('stop', this.stop)
  },
  methods: {
    play() {
      let pc = engine.run_step()
      this.userMem = readEngineMem(engine.ram(), engine.ram_size())
      this.sfrBank = engine.read_sfrs()

      this.$root.$emit('selected-line-update', pc)
    },
    stop() {
      let pc = engine.reset()

      this.userMem = readEngineMem(engine.ram(), engine.ram_size())
      this.sfrBank = engine.read_sfrs()

      this.$root.$emit('selected-line-update', pc)
    },
  }
};
</script>
