<template>
  <div></div>
  <div class="roll">
    <div class="header-bar" :style="{ width: 100 + 5 + 200 * this.n_bars + 'px' }">
      <span
        v-for="beat in 16 + 1"
        :key="'h' + beat"
        :style="{ left: 80 - 3 + (beat - 1) * 200 + 'px' }"
        class="c-label"
      >
        {{ beat - 1 }}
      </span>
    </div>
    <!-- <my-left-bar class="left-bar" :component-name="leftBarStyle" /> -->
    <div class="left-bar">
      <ChannelNames :Range="5" />
    </div>
    <div class="content-area">
      <PatternManager />
    </div>
    <!-- </my-left-bar> -->
  </div>
</template>

<script>
// import Score from './Score.vue'
import PatternManager from './PatternManager.vue'
import ChannelNames from './ChannelNames.vue'
export default {
  name: 'Channels',
  components: {
    // Score,
    ChannelNames,
    PatternManager,
  },
  props: {
    leftBarStyle: {
      type: String,
      default: 'ChannelHeader',
    },
  },
  data() {
    return {
      n_bars: 16,
    }
  },
}
</script>

<style scoped>
.roll {
  position: relative;
  height: 400px;
  width: 100%;
  background: #ccc;
  overflow-x: scroll;
  overflow-y: scroll;
  z-index: 0;

  .content-area {
    left: 80px;
    position: relative;
    width: calc(100%);
    height: calc(100%);
    z-index: 1;
  }

  .left-bar {
    position: sticky;
    left: 0;
    z-index: 2;
  }

  .header-bar {
    position: sticky;
    top: 0;
    height: 20px;
    background: #444;
    z-index: 3;
  }
}
</style>
