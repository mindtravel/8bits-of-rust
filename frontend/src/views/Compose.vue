<template>
  <div>
    <my-button text = "合成器" @click="changeState('synth')"/>|
    <my-button text = "混音台" @click="changeState('mixer')"/>|
    <my-button text = "编曲" @click="changeState('arangement')"/>|
    <my-button text = "导出歌曲" @click="changeState('export-song')"/>
    <br>
    <my-text content="Tips: 用滚轮控制旋钮"/>
    <div class = 'placeholder-block'></div>

    <synth v-if="isState('synth')"></synth>
    <mixer v-if="isState('mixer')"></mixer>
    <arangement v-if="isState('arangement')"></arangement>
    <export-song v-if="isState('export-song')"></export-song>
  </div>
</template>
  
<script>
import MyText from "@/components/utils/MyText.vue";
import MyButton from '@/components/utils/MyButton.vue';
import Synth from '@/components/compose/Synth.vue';
import Mixer from '@/components/compose/Mixer.vue';
import Arangement from '@/components/compose/Arangement.vue';
import ExportSong from '@/components/compose/ExportSong.vue';
export default {
  name: 'Compose',
  components: {
    'my-text': MyText,
    'my-button': MyButton,
    'synth': Synth,
    'mixer': Mixer,
    'arangement': Arangement,
    'export-song': ExportSong,
  },
  methods: {
    changeState(state) {
      this.$store.commit('changeCompose', state);
    },
    isState(state) {
      console.log("compose状态：", this.$store.state.compose);
      return this.$store.state.compose === state;
    },
  }
}
</script>
  
<style>
.placeholder-block {
  width: 300px;       /* 指定宽度 */
  height: 100px;      /* 指定高度 */
}
</style>
  