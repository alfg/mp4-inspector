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
  computed: {
    parsedData() {
      const boxes = [];

      if (this.data && Array.isArray(this.data)) {
        this.data.forEach((o) => {
          Object.keys(o).forEach(p => {
            boxes.push({ name: p, data: o[p] });
          });
        });
      } else if (this.data && typeof this.data === 'object') {
        Object.keys(this.data).forEach(o => {
          boxes.push({ name: o, data: this.data[o] });
        });
      }
      return boxes;
    }
  },
  methods: {
    parsedValue(v) {
      if (typeof v === 'number' || typeof v === 'string') {
        return v;
      } else if (Array.isArray(v) && v.length === 4) { // FourCC value.
        let result = "";
        for (var i = 0; i < v.length; i++) {
          result += String.fromCharCode(v[i]);
        }
        return result;
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
.value {
  padding-left: 4px;
}
</style>
