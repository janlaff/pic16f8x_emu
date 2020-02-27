<template>
  <v-card class="ma-5 flex-grow-1 d-flex flex-column">
    <h5 class="headline ma-3">Program</h5>
    <Controls></Controls>
    <div class="flex-grow-1" style="overflow-y: auto">
      <v-list dense>
        <template v-for="(line, index) in lines">
          <v-list-item :id="'code-' + index" :key="index" class="d-flex">
            <div class="d-flex flex-grow-1">
              <v-list-item-content class="flex-grow-0" style="flex-basis: 50px">{{index}}</v-list-item-content>
              <v-list-item-content class="flex-grow-0" style="flex-basis: 100px">
                {{line.label}}
              </v-list-item-content>
              <v-list-item-content class="flex-grow-1">{{line.info}}</v-list-item-content>
            </div>
          </v-list-item>
        </template>
      </v-list>
    </div>
    <!--<v-list-item v-for="line in lines" :key="line.index">{{line}}</v-list-item>-->
  </v-card>
</template>

<script>
import {engine} from "../emulator"
import Controls from "./Controls"

export default {
  name: "DebugViewer",
  components: { Controls },
  data () {
    return {
      lines: [],
      selectedIndex: 0,
    }
  },
  mounted () {
    this.fetchLines()

    this.$root.$on('selected-line-update', this.selectLine)
  },
  methods: {
    selectLine (index) {
      document.getElementById('code-' + this.selectedIndex).classList.remove('highlighted')
      document.getElementById('code-' + index).classList.add('highlighted')
      document.getElementById('code-' + index).scrollIntoView()
      this.selectedIndex = index
    },
    fetchLines() {
      let count = engine.get_debug_info_line_count()
      let lines = []

      for (let i = 0; i < count; ++i) {
        lines.push({
          info: engine.get_debug_info_line(i),
          label: engine.get_debug_info_line_label(i)
        })
      }

      this.lines = lines
    },
  }
};
</script>

<style>
  .highlighted {
    background-color: lightblue;
  }
</style>
