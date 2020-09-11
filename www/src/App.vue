<template>
  <div>
    <b-navbar type="dark" variant="dark">
      <div class="container">
        <b-navbar-nav>
          <b-nav-item href="#">
            MP4 Inspector
          </b-nav-item>
        </b-navbar-nav>
      </div>
    </b-navbar>

    <GitHubCorner />

    <div id="app" class="container">
      <h3>MP4/ISOBMFF Metadata and Box Structure Viewer</h3>
      <hr />
      <Inspector />
    </div>

    <footer class="container mt-4 text-center">
      <hr />
      <div class="text-muted">
        <ul>
          <li>{{ name }}-{{ version }}</li>
          <li><a href="https://github.com/alfg/mp4-inspector">Source</a></li>
          <li><a href="https://github.com/alfg/mp4-inspector/issues">Report Bugs</a></li>
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
  margin-top: 40px;
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
