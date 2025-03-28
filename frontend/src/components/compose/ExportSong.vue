<template>
  <div>
    <div class="export-params">
      <div class="selector">
        <my-text content="导出格式：" size="large" />
        <my-select
          v-model="exportFormat"
          :options="[
            { label: '.mp3', value: 'mp3' },
            { label: '.wav', value: 'wav' },
          ]"
        />
      </div>
      <div class="selector">
        <my-text content="导出位宽：" size="large" />
        <my-select
          v-model="exportBitWidth"
          :options="[
            { label: '8bit', value: '8bit' },
            { label: '16bit', value: '16bit' },
            { label: '24bit', value: '24bit' },
          ]"
        />
      </div>
      <div class="selector">
        <my-text content="歌曲名字：" size="large" />
        <my-input v-model="songName" size="large" />
      </div>
      <my-text v-bind:content="'预计占用空间：' + estimated_space + 'MB'" size="large" />
    </div>
    <div>
      <my-button size="large" text="导出" />
    </div>
  </div>
</template>
<script>
import { mapState, mapMutations } from 'vuex'
export default {
  computed: {
    ...mapState(['estimated_space', 'songName']),
    songName: {
      get() {
        return this.$store.state.songName
      },
      set(value) {
        this.$store.commit('setSongName', value)
      },
    },
    exportFormat: {
      get() {
        return this.$store.state.exportFormat
      },
      set(value) {
        this.$store.commit('setExportFormat', value)
      },
    },
  },
}
</script>
<style>
.export-params {
  display: inline-block;
  gap: 20px;
  margin-bottom: 15px;
  padding: 10px;
}
.placeholder-block {
  width: 200px; /* 指定宽度 */
  height: 100px; /* 指定高度 */
}
.selector {
  display: flex;
}
</style>
