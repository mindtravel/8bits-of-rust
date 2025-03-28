<template>
  <span class="my-text" :class="[sizeClass]">{{ content }}</span>
</template>

<script>
export default {
  name: 'MyText',
  props: {
    content: String, // 文本内容
    size: {
      type: String, // 文本字体大小
      default: 'medium',
      validator: (v) => ['small', 'medium', 'large'].includes(v),
    },
  },
  computed: {
    sizeClass() {
      return `text-${this.size}`
    },
  },
}
</script>

<style>
@font-face {
  font-family: 'Zpix';
  src: url('@/assets/zpix.ttf') format('truetype');
}
.my-text {
  font-family: 'Zpix', sans-serif;
  color: black;
}

.text-small {
  font-size: 12px;
  line-height: 1.4; /* Adjust this value as needed */
}

.text-medium {
  font-size: 20px;
  line-height: 1.5; /* Adjust this value as needed */
}

.text-large {
  font-size: 30px;
  line-height: 1.6; /* Adjust this value as needed */
}
</style>
