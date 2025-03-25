<template>
  <button 
    class="my-button"
    :class="[sizeClass, { 'disabled': disabled }]"
    :disabled="disabled"
    :aria-label="ariaLabel || text"
    @click="$emit('click', $event)"
    @keydown.enter="$emit('click', $event)"
  >
    <span class="content-wrapper">
      <!-- 插槽支持富文本内容 -->
      <slot name="icon"></slot>
      <span>
        <mytext v-bind:content="text" />
      </span>
    </span>
  </button>
</template>

<script>
import MyText from '@/components/utils/MyText.vue'
export default {
  name: 'MyButton',
  components: {
    'mytext': MyText
  },
  props: {
    text: {
      type: String,
      default: 'Button'
    },
    // 新增样式控制参数
    variant: {
      type: String,
      default: 'primary',
      validator: (v) => ['primary', 'secondary', 'ghost'].includes(v)
    },
    size: {
      type: String,
      default: 'medium',
      validator: (v) => ['small', 'medium', 'large'].includes(v)
    },
    disabled: Boolean,
    ariaLabel: String
  },
  emits: ['click'],
  computed: {
    sizeClass() {
      return `size-${this.size}`
    }
  }
}
</script>

<style scoped>
/* 基础按钮样式 */
.my-button {
  --primary-color: white;
  --secondary-color: rgb(240,240,240);
  --ghost-color: #666;
  
  position: relative;
  border: none;
  /* border-radius: 4px; */
  cursor: pointer;
  transition: 
    background-color 0.2s ease,
    transform 0.1s cubic-bezier(0.25, 0.46, 0.45, 0.94);
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
}

/* 尺寸系统 */
.size-small { padding: 8px 16px; font-size: 14px; }
.size-medium { padding: 12px 24px; font-size: 16px; }
.size-large { padding: 16px 32px; font-size: 18px; }

/* 颜色变体 */
.my-button:not(.disabled) {
  background-color: var(--primary-color);
  color: white;
}
.my-button.secondary:not(.disabled) {
  background-color: var(--secondary-color);
}
.my-button.ghost:not(.disabled) {
  background-color: transparent;
  color: var(--ghost-color);
  border: 1px solid var(--ghost-color);
}

/* 交互状态 */
.my-button:not(.disabled):hover {
  background-color: var(--secondary-color);
}
.my-button:not(.disabled):active {
  transform: scale(0.98);
}

/* 禁用状态 */
.my-button.disabled {
  background-color: #e0e0e0;
  color: #9e9e9e;
  cursor: not-allowed;
  opacity: 0.7;
}

/* 内容布局 */
.content-wrapper {
  display: inherit;
  align-items: inherit;
  gap: inherit;
}
</style>