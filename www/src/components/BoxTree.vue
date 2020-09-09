<template>
  <div class="box-tree">
    <ul>
      <li v-for="(o, i) in parsedData" :key="i">
        <span class="key">{{ o.name.replace('value', '') }}</span>
        <code class="value">{{ parsedValue(o.data) }}</code>
        <template>
          <BoxTree :data="o.data" />
        </template>
      </li>
    </ul>
  </div>
</template>

<script>
export default {
  name: 'BoxTree',
  props: ['data'],
  data() {
    return {
    };
  },
  computed: {
    parsedData() {
      const boxes = [];

      if (Array.isArray(this.data)) {
        this.data.forEach(o => {
          const obj = {
            name: Object.keys(o)[0],
            data: o[Object.keys(o)[0]],
          };
          boxes.push(obj);
        });
      } else if (typeof this.data !== 'object') {
        return; // skip.
      } else {
        Object.keys(this.data).forEach(o => {
          const obj = {
            name: o,
            data: this.data[o],
          };
          boxes.push(obj);
        });
      }
      return boxes;
    }
  },
  methods: {
    parsedValue(v) {
      if (typeof v === 'number' || typeof v === 'string') {
        return v;
      }
      return;
    },
  }
}
</script>

<style scoped>
.key {
  font-weight: bold;
}
</style>
