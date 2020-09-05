<template>
  <div>
    <b-navbar type="dark" variant="dark">
      <b-navbar-nav>
        <b-nav-item href="#">
          MP4 Inspector
        </b-nav-item>
      </b-navbar-nav>
    </b-navbar>

    <GitHubCorner />

    <div id="app" class="container">
      <div class="mt-2">
        <b-alert
          variant="warning"
          show
        >⚠️ Currently a work in progress. Check back for updates.</b-alert>
      </div>
      <Inspector />
    </div>

    <footer class="container mt-4 text-center">
      <hr />
      <div class="text-muted">
        <ul>
          <li>{{ name }}-{{ version }}</li>
          <li><a href="https://github.com/alfg/mp4-inspector">GitHub</a></li>
        </ul>
      </div>
    </footer>
  </div>
</template>

<script>
import { name, version } from '../package.json';
import GitHubCorner from './components/GitHubCorner.vue';
import Inspector from './components/Inspector.vue';

export default {
  name: 'App',
  components: {
    Inspector,
    GitHubCorner,
  },
  data() {
    return {
      name,
      version,
    }
  }
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
  margin-top: 60px;
}

footer ul {
  display: inline-block;
  padding-left: 0;
  text-align: left;
  width: 100%;
}

footer ul li {
  display: inline;
  margin: 0 6px;
  list-style: none;
}
</style>
