<template>
  <div class="ma-5">
    <v-btn rounded color="primary" @click="$root.$emit('play')" class="mr-1">
      <v-icon color="white">mdi-play</v-icon>Run
    </v-btn>
    <v-btn rounded color="primary" @click="$root.$emit('step')" class="mr-1">
      <v-icon color="white">mdi-step-forward</v-icon>Step
    </v-btn>
    <v-btn rounded color="primary" @click="$root.$emit('stop')" class="mr-1">
      <v-icon color="white">mdi-stop</v-icon>Abort
    </v-btn>
    <v-btn rounded color="primary" @click="$root.$emit('folder')">
      <v-icon color="white">mdi-folder</v-icon>Load file
    </v-btn>
  </div>
</template>

<script>
export default {
  name: "Controls"
};
</script>
