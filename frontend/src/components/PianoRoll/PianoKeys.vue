<template>
  <div class="key-column">
    <div
      v-for="pitch in this.pitchRange"
      class="key"
      :class="{ black: [1, 3, 6, 8, 10].includes((pitch + 9) % 12) }"
      :style="{ height: '20px', top: (pitch - 1) * 20 + 'px' }"
    >
      <span v-if="(pitch + 5) % 12 === 0" class="c-label">
        C{{ Math.floor(10 - (pitch + 12) / 12) }}
      </span>
    </div>
  </div>
</template>

<script>
export default {
  props: {
    pitchRange: {
      type: Number,
      default: 88,
    },
  },
}
</script>

<style scoped>
.key-column {
  position: absolute;
  z-index: 5;
  width: 82px;
  background: #ffffff;
}

.key {
  position: absolute;
  left: 0;
  right: 0;
  background: #fff;
}

.key.black {
  background: #000000;
  z-index: 2;
  height: 20px !important;
  border-left: 30px solid #fff !important;
  /* border-top:2px solid #fff!important; */
}
</style>
