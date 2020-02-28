<template>
  <v-btn-toggle class="ma-5">
    <v-btn fab small color="primary" @click="$root.$emit('play')">
      <v-icon color="white">mdi-play</v-icon>
    </v-btn>
    <v-btn fab small color="primary" @click="$root.$emit('step')">
      <v-icon color="white">mdi-step-forward</v-icon>
    </v-btn>
    <v-btn fab small color="primary" @click="$root.$emit('stop')">
      <v-icon color="white">mdi-stop</v-icon>
    </v-btn>
    <v-btn fab small color="primary" @click="$root.$emit('folder')">
      <v-icon color="white">mdi-folder</v-icon>
    </v-btn>
    <!--<b-button @click="$emit('play')" variant="success">
            <font-awesome-icon icon="play"></font-awesome-icon>
        </b-button>
        <b-button @click="$emit('stop')" variant="danger">
            <font-awesome-icon icon="stop"></font-awesome-icon>
        </b-button>
        <b-button @click="$emit('file')" variant="primary">
            <font-awesome-icon icon="folder"></font-awesome-icon>
    </b-button>-->
  </v-btn-toggle>
</template>

<script>
export default {
  name: "Controls"
};
</script>
