<template>
  <div>
    <div class="main-container">
      <channels ref="channelSection" />
    </div>
    <br />
    <div class="main-container">
      <piano-roll :notes="testNotes" />
    </div>
  </div>
</template>
<script>
import PianoRoll from '../PianoRoll/PianoRoll.vue'
import Channels from '../Channels/Channels.vue'

export default {
  components: {
    'piano-roll': PianoRoll,
    'channels': Channels,
  },
  // computed: {
  //     viewportWidth() {
  //         return this.$store.state.scrollX;
  //     }
  // },
  // methods: {
  //     handleTimelineScroll(e) {
  //         this.$store.commit('setScrollPosition', {
  //             x: e.target.scrollLeft,
  //             y: this.$store.state.scrollY
  //         });
  //     }
  // },
  // mounted() {
  //     document.addEventListener('mousemove', this.handleResizeMove);
  //     document.addEventListener('mouseup', this.stopResize);
  // },
  // beforeDestroy() {
  //     document.removeEventListener('mousemove', this.handleResizeMove);
  //     document.removeEventListener('mouseup', this.stopResize);
  // },
  methods: {
    // startResize(e) {
    //     this.isDragging = true;
    //     this.initialX = e.clientX;
    //     this.initialWidth = this.$refs.channelSection.offsetWidth;
    // },
    // syncScroll() {
    //     this.$refs.pianoSection.scrollLeft = this.$refs.channelSection.scrollLeft;
    // }
  },
}
</script>
<style></style>
