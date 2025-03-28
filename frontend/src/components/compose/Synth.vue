<template>
  <my-text v-bind:content="'波形预设:' + preset" size="large" /><br />

  <my-text v-bind:content="'音量包络'" size="large" />
  <my-text v-bind:content="'起音：'" /><my-knob class="label" />
  <my-text v-bind:content="'/ms  衰减：'" /><my-knob class="label" />
  <my-text v-bind:content="'/ms  保持：'" /><my-knob class="label" />
  <my-text v-bind:content="'/ms  释放：'" /><my-knob class="label" />
  <my-text v-bind:content="'/ms'" /><br />

  <my-text v-bind:content="'FM调制:'" size="large" />
  <my-text v-bind:content="'幅度：'" /><my-knob class="label" />
  <my-text v-bind:content="'/semitone  速率：'" /><my-knob class="label" />
  <my-text v-bind:content="'/Hz'" />
</template>
<script>
import MyText from '@/components/utils/MyText.vue'
import MyButton from '@/components/utils/MyButton.vue'
import MyKnob from '@/components/utils/MyKnob.vue'
export default {
  components: {
    'my-text': MyText,
    'my-button': MyButton,
    'my-knob': MyKnob,
  },
  data() {
    return {
      preset: 'default',
      attack: 0,
      decay: 0,
      sustain: 0,
      release: 0,
    }
  },
}
</script>
<style>
.label {
  display: inline-block;
}
</style>
