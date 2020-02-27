<template>
    <div class="sfr-viewer">
        <label>Status</label>
        {{bank.status}}
    </div>
</template>

<script>

export default {
  name: 'SFRViewer',
  props: ['bank'],
  data () {
    return {}
  }
}

</script>
