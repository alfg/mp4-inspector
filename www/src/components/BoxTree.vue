<template>
  <div>
    <ul>
      <li v-for="(o, i) in parsedData" :key="i">
        {{ o.name }}

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
        // console.log('not object', this.data);
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
  }
}
</script>

<style scoped>
.bold {
  font-weight: bold;
}
ul {
  /* padding-left: 1em; */
  /* line-height: 1.5em; */
}
li {
  cursor: pointer;
}

li:hover {
  /* font-weight: bold; */
}
</style>
