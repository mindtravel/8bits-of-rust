<template>
  <div>
    <my-button text="插件" :active="$store.state.activeComposePage === 'plugin'" @click="changeComposePage('plugin')"/>|
    <my-button text="混音台" :active="$store.state.activeComposePage === 'mixer'" @click="changeComposePage('mixer')"/>|
    <my-button text="编曲" :active="$store.state.activeComposePage === 'arrangement'" @click="changeComposePage('arrangement')"/>|
    <my-button text="导出歌曲" :active="$store.state.activeComposePage === 'export'" @click="changeComposePage('export')"/>
    <br>
    <my-text content="Tips: 用滚轮控制旋钮"/>
    <div class = 'placeholder-block'></div>

    <plugin v-if="isComposePage('plugin')"></plugin>
    <mixer v-if="isComposePage('mixer')"></mixer>
    <arrangement v-if="isComposePage('arrangement')"></arrangement>
    <export-song v-if="isComposePage('export')"></export-song>
  </div>
</template>
  
<script>

import Plugin from '@/components/compose/Plugin.vue';
import Mixer from '@/components/compose/Mixer.vue';
import Arrangement from '@/components/compose/Arrangement.vue';
import ExportSong from '@/components/compose/ExportSong.vue';

export default {
  components: {
    'plugin': Plugin,
    'mixer': Mixer,
    'arrangement': Arrangement,
    'export-song': ExportSong,
  },
  methods: {
    isComposePage(state) {
      console.log("compose状态：", this.$store.state.activeComposePage);
      return this.$store.state.activeComposePage === state;
    },
    changeComposePage(state, newState){
      this.$store.commit('setActiveComposePage', state, newState);
    }
  },
  data() {
    return {
      currentState: this.$store.state.activeComposePage || 'plugin',
      testNotes: [
        { pitch: 60, start: 1, duration: 2 },
        { pitch: 64, start: 3, duration: 1 },
        { pitch: 67, start: 5, duration: 3 }
      ]
    };
  }
}
</script>
  
<style>
.placeholder-block {
  height: 50px;      /* 指定高度 */
}
</style>
  
<my-section title="钢琴卷帘">
  <piano-roll :notes="testNotes" />
</my-section>
  