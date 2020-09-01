<template>
  <div>
    <h4>Info</h4>
    <b-table stacked :items="items"></b-table>

    <h4>Tracks</h4>
    <b-table striped hover :items="tracks"></b-table>
  </div>
</template>

<script>
export default {
  name: 'Overview',
  props: ['tracks', 'info'],
  data() {
    return {
      items: [
        {
          size: this.info.size,
          major_brand: this.info.major_brand,
          minor_version: this.info.minor_version,
          compatible_brands: this.info.compatible_brands,
          timescale: this.info.timescale,
          duration: this.info.duration,
        },
      ]
    }
  }
}
</script>
