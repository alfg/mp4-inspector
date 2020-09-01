<template>
    <div>
        <h2>Select a file:</h2>
        <b-form-file
            accept="video/mp4"
            v-model="file"
            :state="Boolean(file)"
            placeholder="Choose a file or drop it here..."
            drop-placeholder="Drop file here..."
            @change="onFile"
        ></b-form-file>
        <div class="mt-3">Selected file: {{ file ? `${file.name}: ${file.size} bytes` : '' }}</div>

        <div v-if="data">
          <h2>Results</h2>
          <!-- MP4 Boxes -->
          <b-table striped hover :items="mp4data"></b-table>

          <!-- MP4 Tracks -->
          <b-table striped hover :items="tracks"></b-table>
        </div>
    </div>

</template>

<script>
export default {
  name: 'Inspector',
  data() {
    return {
      file: null,
      data: null,
    }
  },
  computed: {
    mp4data() {
      return this.$mp4.get_boxes(this.data);
    },
    tracks() {
      return this.$mp4.get_tracks(this.data);
    }
  },
  methods: {
    onFile(event) {
      const file = event.target.files[0];
      const reader = new FileReader();

      // reader.onload = e => this.$emit("load", event.target.result);
      reader.onload = (event) => {
        this.data = new Uint8Array(event.target.result);
      }
      reader.readAsArrayBuffer(file);
    }
  }
}
</script>
