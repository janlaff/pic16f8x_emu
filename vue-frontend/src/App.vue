<template>
  <div id="app">
    <span>{{value}}</span>
  </div>
</template>

<script>
import {ram} from "./emulator";

export default {
  name: 'App',
  data () {
    return {
      value: ram[0]
    }
  },
}
</script>

<style lang="scss">
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
