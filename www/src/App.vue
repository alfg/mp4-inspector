<template>
  <div id="app" class="container">
    <h1>MP4 Inspector</h1>
    <Inspector />
  </div>
</template>

<script>
import Inspector from './components/Inspector.vue';

export default {
  name: 'App',
  components: {
    Inspector,
  },
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
