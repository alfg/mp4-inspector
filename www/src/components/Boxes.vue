<template>
  <div>
    <b-table striped hover :items="boxes"></b-table>
  </div>
</template>

<script>
export default {
  name: 'Boxes',
  props: ['boxes'],
}
</script>
