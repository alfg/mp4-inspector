<template>
    <div class="inspector">
        <b-form-group label="Select a file:" label-for="file">
          <b-form-file
              id="file"
              accept="video/mp4"
              v-model="file"
              :state="Boolean(file)"
              placeholder="Choose a file or drop it here..."
              drop-placeholder="Drop file here..."
              @change="onFile"
          ></b-form-file>
        </b-form-group>

        <b-progress
          height="2px"
          v-if="showProgress"
          :value="progress"
          max="100"></b-progress>

        <div v-if="data">
          <div class="mt-3">Selected file: {{ file ? `${file.name}: ${file.size} bytes` : '' }}</div>

          <b-tabs class="mt-4">
            <b-tab title="Overview" class="mt-2">
              <div v-if="data">
                <Overview :tracks="tracks" :info="info" />
              </div>
            </b-tab>

            <b-tab title="Boxes" class="mt-2">
              <div v-if="data">
                <Boxes :boxes="boxes" />
              </div>
            </b-tab>
          </b-tabs>
          <hr />
        </div>
    </div>
</template>

<script>
import Overview from './Overview.vue';
import Boxes from './Boxes.vue';

export default {
  name: 'Inspector',
  components: {
    Overview,
    Boxes,
  },
  data() {
    return {
      file: null,
      data: null,
      progress: 0,
      showProgress: false,
    }
  },
  computed: {
    info() {
      return this.$mp4.get_media_info(this.data);
    },
    tracks() {
      return this.$mp4.get_tracks(this.data);
    },
    boxes() {
      return this.$mp4.get_boxes(this.data);
    },
  },
  methods: {
    onFile(event) {
      this.data = null;
      this.progress = 0;
      this.showProgress = true;
      const file = event.target.files[0];
      const reader = new FileReader();

      // reader.onload = e => this.$emit("load", event.target.result);
      reader.onload = (event) => {
        this.progress = 100;
        this.data = new Uint8Array(event.target.result);
        setTimeout(() => { this.showProgress = false; }, 2000);
      }
      reader.onprogress = (event) => {
        if (event.lengthComputable) {
          this.progress = parseInt(((event.loaded / event.total) * 100), 10);
        }
      }
      reader.readAsArrayBuffer(file);
    }
  }
}
</script>
