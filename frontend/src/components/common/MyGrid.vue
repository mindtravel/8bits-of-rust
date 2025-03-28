<template>
  <div
    class="grid"
    :style="{
      width: 25 * 8 * this.n_bars + 25 + 'px',
      height: this.h_rows * this.n_rows + 1 + 'px',
    }"
  >
    <div
      v-for="beat in 8 * this.n_bars"
      class="grid-horizontal"
      :style="{ left: beat * 25 - 1 + 'px' }"
    ></div>

    <div
      v-for="i in this.n_rows + 1"
      class="grid-vertical"
      :style="{ top: (i - 1) * this.h_rows - 1 + 'px' }"
    ></div>
  </div>
</template>

<script>
export default {
  name: 'MyGrid',
  props: {
    n_bars: {
      type: Number,
      default: 16,
    },
    n_rows: {
      type: Number,
      default: 5,
    },
    h_rows: {
      type: Number,
      default: 60,
    },
  },
}
</script>

<style>
.grid {
  position: relative;
  height: 100%;
  background-color: #ccc;
}

.grid-horizontal {
  position: absolute;
  width: 1px;
  height: 100%;
  border-left: 2px solid #aaa;
  /* z-index: 10; */
}

.grid-vertical {
  position: absolute;
  height: 1px;
  width: 100%;
  border-top: 2px solid #aaa;
}
</style>
