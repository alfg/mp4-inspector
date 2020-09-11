<template>
  <div class="samples">
    <b-row class="mt-4">
      <b-col cols="12" md="4">
        <b-form-select
          v-model="selectedTrack"
          :options="tracks"
        >
        </b-form-select>
      </b-col>
    </b-row>

    <b-table
      class="mt-4"
      striped hover
      :items="samples[selectedTrack - 1].samples"
      :per-page="perPage"
      :current-page="currentPage"
    ></b-table>

    <b-row class="mt-4">
      <b-col cols="12" md="10">
        <b-pagination
          v-model="currentPage"
          :total-rows="pages"
          :per-page="perPage"
        ></b-pagination>
      </b-col>
      <b-col cols="12" md="2">
        <b-form-select
          v-model="perPage"
          :options="options"
          v-on:change="getSelectedItem">
        </b-form-select>
      </b-col>
    </b-row>
  </div>
</template>

<script>
export default {
  name: 'Samples',
  props: ['samples'],
  data() {
    return {
      selectedTrack: 1,
      currentPage: 1,
      perPage: 12,
      options: [
        { value: 12, text: '12' },
        { value: 24, text: '24' },
        { value: 48, text: '48' }
      ],
    };
  },
  computed: {
    tracks() {
      return this.samples.map((o) => {
        return {
          text: `Track ID: ${o.track_id} (${o.box_type}) -- ${o.samples.length} Samples`,
          value: o.track_id };
      });
    },
    pages() {
      return this.samples[this.selectedTrack - 1].samples.length;
    },
  },
  methods: {
    linkGen(pageNum) {
      return pageNum === 1 ? '?' : `?page=${pageNum}`;
    },
    getSelectedItem(event) {
      this.perPage = event;
    },
  }
}
</script>
