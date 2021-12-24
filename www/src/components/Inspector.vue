<template>
    <div class="inspector">
      <b-form-row>
        <b-col>
          <b-form-group label="Select a file:" label-for="file">
            <b-input-group>
              <b-form-select class="protocol" v-model="protocol">
                <option v-for="o in protocols" :key="o.id" :value="o.value">{{ o.name }}</option>
              </b-form-select>

              <b-form-file
                  v-if="protocol === 'file'"
                  id="file"
                  accept="video/mp4"
                  v-model="file"
                  :state="Boolean(file)"
                  placeholder="Choose a file or drop it here..."
                  drop-placeholder="Drop file here..."
                  @change="onFile"
              ></b-form-file>

              <b-form-input
                v-if="protocol === 'url'"
                v-model="url"
                :state="Boolean(url)"
                placeholder="Enter a URL"
              ></b-form-input>

              <b-form-select v-if="protocol === 'example'" v-model="url">
                <template #first>
                  <b-form-select-option :value="null" disabled>-- Please select an option --</b-form-select-option>
                </template>
                <option v-for="o in examples" :key="o.id" :value="o.value">{{ o.name }}</option>
              </b-form-select>

              <b-input-group-append v-if="protocol !== 'file'">
                <b-button @click="onDownload">Download</b-button>
              </b-input-group-append>
            </b-input-group>
          </b-form-group>
        </b-col>
      </b-form-row>

        <b-progress
          height="2px"
          v-if="showProgress"
          :value="progress"
          max="100">
        </b-progress>

        <div v-if="data">
          <div class="mt-3" v-if="file">Selected file: {{ file ? `${file.name}: ${file.size} bytes` : '' }}</div>
          <div class="mt-3" v-else>Selected file: {{ url ? `${url} (${data.length} bytes)` : '' }}</div>

          <b-tabs class="mt-4">
            <b-tab title="Overview" class="mt-2">
              <div v-if="data">
                <Overview :tracks="tracks" :info="info" />
              </div>
            </b-tab>

            <b-tab title="Box Tree" class="mt-2">
              <div class="tree-view" v-if="data">
                <BoxTree :data="boxes" />
              </div>
            </b-tab>

            <b-tab title="Samples" class="mt-2">
              <div v-if="data">
                <Samples :samples="samples" />
              </div>
            </b-tab>
          </b-tabs>
          <hr />
        </div>
    </div>
</template>

<script>
import Overview from './Overview.vue';
import BoxTree from './BoxTree.vue';
import Samples from './Samples.vue';

export default {
  name: 'Inspector',
  components: {
    Overview,
    BoxTree,
    Samples,
  },
  data() {
    return {
      protocols: [
        { name: 'File', value: 'file'},
        { name: 'URL', value: 'url'},
        { name: 'Example', value: 'example'},
      ],
      examples: [
        { name: 'Video Counter (10min, unfragmented, AVC Baseline)', value: 'https://video-examples-public.s3.us-west-2.amazonaws.com/video_counter_10min_unfragmented_avc.mp4' },
        { name: 'Tears of Steel 360p (00:12:14, unfragmented, AVC Baseline, 67.85 MB)', value: 'https://video-examples-public.s3.us-west-2.amazonaws.com/tears-of-steel-360p.mp4' },
      ],
      protocol: 'file',
      file: null,
      url: null,
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
      const boxes = this.$mp4.get_boxes(this.data);
      const data = [];
      boxes.forEach(o => {
        const obj = {
          [o.name]: JSON.parse(o.json),
        };
        data.push(obj);
      });
      return data;
    },
    samples() {
      return this.$mp4.get_samples(this.data);
    },
  },
  methods: {
    onFile(event) {
      this.data = null;
      this.progress = 0;
      this.showProgress = true;

      const file = event.dataTransfer ? event.dataTransfer.files[0] : event.target.files[0];
      const reader = new FileReader();

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
    },
    onDownload() {
      this.data = null;
      this.progress = 0;
      this.showProgress = true;

      const reader = new FileReader();
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

      const xhr = new XMLHttpRequest();
      xhr.onprogress = (event) => {
        if (event.lengthComputable) {
          this.progress = parseInt(((event.loaded / event.total) * 100), 10);
        }
      }
      xhr.onload = (event) => {
        this.progress = 100;
        reader.readAsArrayBuffer(event.target.response);
      }
      xhr.open('GET', this.url, true);
      xhr.responseType = 'blob';
      xhr.send();
    },
  }
}
</script>

<style scoped>
.protocol {
  flex: 0 0 20% !important;
}
.tree-view {
  overflow: auto;
  height: 60vh;
}
</style>
