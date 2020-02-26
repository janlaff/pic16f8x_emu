<template>
    <div class="memory-viewer">
        <ul>
            <li v-for="(value, index) in buffer">
                {{index}}: {{value}}
            </li>
        </ul>
    </div>
</template>

<script>

export default {
  name: 'MemoryViewer',
  props: ['buffer'],
}

</script>
