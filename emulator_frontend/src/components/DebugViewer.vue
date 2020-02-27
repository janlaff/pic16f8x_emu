<template>
  <v-card class="ma-5 flex-grow-1 d-flex flex-column">
    <h5 class="headline ma-3">Program</h5>
    <div class="flex-grow-1" style="overflow-y: auto">
      <v-list>
        <v-simple-table>
          <template v-slot:default>
            <tbody>
            <tr v-for="(line, index) in lines" :key="line.index">
              <td>{{index}}</td>
              <td>{{line.label}}</td>
              <td>{{line.info}}</td>
            </tr>
            </tbody>
          </template>
        </v-simple-table>
      </v-list>
    </div>
    <!--<v-list-item v-for="line in lines" :key="line.index">{{line}}</v-list-item>-->
  </v-card>
</template>

<script>
import {engine} from "../emulator"

export default {
  name: "DebugViewer",
  data () {
    return {
      lines: []
    }
  },
  mounted () {
    this.fetchLines()
  },
  methods: {
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
    }
  }
};
</script>
