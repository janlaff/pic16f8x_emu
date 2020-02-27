<template>
    <div class="controls">
        <v-btn fab dark small color="primary" @click="$emit('play')">
            <font-awesome-icon icon="play"></font-awesome-icon>
        </v-btn>
        <v-btn fab dark small color="deep-purple accent-4" @click="$emit('stop')">
            <font-awesome-icon icon="stop"></font-awesome-icon>
        </v-btn>
        <v-btn fab dark small color="secondary" @click="$emit('folder')">
            <font-awesome-icon icon="folder"></font-awesome-icon>
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
    </div>
</template>

<script>

export default {
  name: 'Controls'
}

</script>
