<template>
  <div class="left-bar">
    <slot></slot>
  </div>
</template>

<script>
export default {
  functional: true,
  props: {
    componentName: {
      type: String,
      default: 'div',
    },
  },
  render(createElement, { props, children }) {
    return createElement(
      props.componentName,
      {
        class: 'left-bar',
      },
      children
    )
  },
}
</script>
<style scoped>
.left-bar {
  width: 80px;
  background: #444;
  min-height: 1600px;
  z-index: 5;
  transform: translateZ(0);
}
</style>
