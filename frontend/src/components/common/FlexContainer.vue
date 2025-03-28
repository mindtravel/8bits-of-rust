<template>
  <div
    class="flex-container"
    :style="{
      display: layoutType,
      flexDirection: direction,
      alignItems: align,
      justifyContent: justify,
      gap: gap + 'px',
    }"
  >
    <slot></slot>
  </div>
</template>

<script>
export default {
  name: 'FlexContainer',
  props: {
    layoutType: {
      type: String,
      default: 'flex',
      validator: (value) => ['flex', 'grid'].includes(value),
    },
    direction: {
      type: String,
      default: 'row',
    },
    align: {
      type: String,
      default: 'stretch',
    },
    justify: {
      type: String,
      default: 'flex-start',
    },
    gap: {
      type: Number,
      default: 8,
    },
  },
}
</script>

<style scoped>
.flex-container {
  width: 100%;
  height: 100%;
  padding: 8px;
  border: 1px solid #e0e0e0;
  border-radius: 4px;
  background-color: rgba(255, 255, 255, 0.9);
}
</style>
