<template>
<div>
  <div class="main-container">
    <track-sections ref="trackSection"/>
  </div>
  <br>
  <div class="main-container">
    <piano-roll :notes="testNotes"/>
  </div>
</div>
</template>
<script>
import PianoRoll from '../Window/PianoRoll.vue';
import TrackSections from './TrackSections.vue';

export default {
    name: 'Arrangement',
    components: {
        'piano-roll': PianoRoll,
        'track-sections': TrackSections
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
        //     this.initialWidth = this.$refs.trackSection.offsetWidth;
        // },
        // syncScroll() {
        //     this.$refs.pianoSection.scrollLeft = this.$refs.trackSection.scrollLeft;
        // }
    }
}

</script>
<style>

</style>