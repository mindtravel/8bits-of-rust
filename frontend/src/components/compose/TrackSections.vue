<template>
  <my-roll>
    <!-- <my-left-bar class="left-bar"> -->
      <my-grid class="grid"/>
    <!-- </my-left-bar> -->
      <!-- <div class="track-labels"> -->
        <!-- <div 
          v-for="track in this.$store.state.tracks"
          class="track-label my-text"
          :style="{ top: ((track.id - 1) * 60) + 'px' }"
        >
          {{ track.name }}
        </div> -->
        <!-- <my-grid @mousedown="handleGridClick" @contextmenu.prevent=""> -->
          <!-- ɾ��ԭ���ظ���key-column�ṹ -->
          <!-- <div
            v-for="(pattern, index) in $store.state.patterns"
            :key="index"
            class="pattern"
            :style="{
              left: (80 + pattern.start * 25) + 'px',
              top: ((pattern.track - 1) * 20) + 'px',
              width: (pattern.duration * 25) + 'px'
            }"
            @mousedown="handleResizeStart(index, $event)"
          ></div> -->
          <!-- <div class="resize-handle" @mousedown.stop.prevent></div> -->
      <!-- </div> -->

      
  </my-roll>
</template>
  
<script>
export default {
    name: 'TrackSections',
    props: {
      patterns: {
        type: Array,
        default: () => []
      }
    },
    // mounted() {
    //   this.$refs.timeline.addEventListener('scroll', this.syncHeaderScroll);
    // },
    // beforeDestroy() {
    //   this.$refs.timeline.removeEventListener('scroll', this.syncHeaderScroll);
    // },
    // methods: {
    //   syncHeaderScroll(e) {
    //     const header = this.$el.querySelector('.header-bar');
    //     if(header) header.scrollLeft = e.target.scrollLeft;
    //     this.$store.commit('setScrollPosition', { 
    //       x: e.target.scrollLeft,
    //       y: e.target.scrollTop
    //     });
    //   },
    //   handleGridClick(e) {
    //     e.preventDefault();
    //     const rect = this.$refs.timeline.getBoundingClientRect();
    //     const track = Math.floor((e.clientY - rect.top) / 20);
    //     const beat = Math.floor((e.clientX - rect.left - 80) / 25);
        
    //     if (e.button === 0) { // ���
    //       const existingIndex = this.$store.state.patterns.findIndex(n => 
    //         n.track === track && n.start === beat
    //       );
          
    //       if (existingIndex > -1) {
    //         // �����������϶��߼�
    //         const startX = e.clientX;
    //         const originalDuration = this.$store.state.patterns[existingIndex].duration;
            
    //         const moveHandler = (moveE) => {
    //           const delta = (moveE.clientX - startX) / 25;
    //           const newDuration = Math.max(1, originalDuration + delta);
    //           this.$store.commit('updatepatternDuration', {
    //             index: existingIndex,
    //             duration: Math.floor(newDuration)
    //           });
    //         };
            
    //         const upHandler = () => {
    //           document.removeEventListener('mousemove', moveHandler);
    //           document.removeEventListener('mouseup', upHandler);
    //         };
            
    //         document.addEventListener('mousemove', moveHandler);
    //         document.addEventListener('mouseup', upHandler);
    //       } else {
    //         // ������������������ק
    //         const creatingpattern = {
    //           track,
    //           start: beat,
    //           duration: 1,
    //           velocity: 100
    //         };
    //         const index = this.$store.state.patterns.length;
    //         this.$store.commit('addpattern', creatingpattern);
            
    //         const startX = e.clientX;
    //         const moveHandler = (moveE) => {
    //           const newDuration = Math.max(1, Math.floor((moveE.clientX - startX) / 25));
    //           this.$store.commit('updatepatternDuration', {
    //             index,
    //             duration: Math.floor(newDuration)
    //           });
    //         };
            
    //         const upHandler = () => {
    //           document.removeEventListener('mousemove', moveHandler);
    //           document.removeEventListener('mouseup', upHandler);
    //         };
            
    //         document.addEventListener('mousemove', moveHandler);
    //         document.addEventListener('mouseup', upHandler);
    //       }
    //     } else if (e.button === 2 && !e.ctrlKey) { // �Ҽ�
    //       const index = this.$store.state.patterns.findIndex(n => 
    //         n.track === track && n.start <= beat && n.start + n.duration > beat
    //       );
    //       if (index > -1) this.$store.commit('deletepattern', index);
    //     }
    //   }
    // }
  }
  </script>
  
  <style scoped>
  
  .track-labels {
    position: sticky;
    left: 0;
    z-index: 2;
    width: 80px;
    background: #f5f5f5;
    border-right: 2px solid #ddd;
    height: 100%;
  }
  
  .track-label {
    display: flex;
    align-items: center;
    justify-content: center;
    position: absolute;
    margin-top: 20px;
    left: 0;
    right: 0;
    color: #666;
    border-bottom: 1px solid #eee;
  }
  
  .pattern {
    position: absolute;
    height: 20px;
    width: 25px;
    background: #8f8;
    box-sizing: border-box;
    opacity: 0.8;
    user-select: none;
    -webkit-user-select: none;
  }

  /* .left-bar {
    position: fixed;
  } */

  .grid{
    left :80;
  }
</style>