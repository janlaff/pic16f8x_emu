<template>
    <v-data-table :headers="headers" :items="items" :items-per-page="5" class="sfr-viewer elevation-1"></v-data-table>
</template>

<script>

export default {
  name: 'SFRViewer',
  props: ['bank'],
  data () {
    return {
      headers: [
        {
          text: 'Register',
          value: 'register'
        },
        {
          text: 'Value',
          value: 'value'
        }
      ],
      items: [
        {
          register: 'Status',
          value: 0,
        }
      ]
    }
  },
  watch: {
    bank: function (val) {
      this.items = [
        {
          register: 'Status',
          value: this.bank.status,
        }
      ]
    }
  }
}

</script>
