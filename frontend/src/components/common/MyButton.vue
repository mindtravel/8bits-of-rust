<template>
  <button
    class="my-button"
    :class="[sizeClass, { disabled: disabled, active: active }]"
    :disabled="disabled"
    :aria-label="ariaLabel || text"
    @click="$emit('click', $event)"
    @keydown.enter="$emit('click', $event)"
  >
    <span class="content-wrapper">
      <slot name="icon"></slot>
      <span>
        <my-text v-bind:content="text" />
      </span>
    </span>
  </button>
</template>

<script>
export default {
  name: 'MyButton',
  components: {},
  props: {
    active: {
      type: Boolean,
      default: false,
    },
    text: {
      type: String,
      default: 'Button',
    },
    variant: {
      type: String,
      default: 'primary',
      validator: (v) => ['primary', 'secondary', 'ghost'].includes(v),
    },
    size: {
      type: String,
      default: 'medium',
      validator: (v) => ['small', 'medium', 'large'].includes(v),
    },
    disabled: Boolean,
    ariaLabel: String,
  },
  emits: ['click'],
  computed: {
    sizeClass() {
      return `size-${this.size}`
    },
  },
}
</script>

<style scoped>
.my-button {
  --primary-color: rgb(255, 255, 255);
  --secondary-color: rgb(240, 240, 240);
  --ghost-color: var(--pixel-ghost);

  position: relative;
  border: none;
  /* border-radius: 4px; */
  cursor: pointer;
  background-color: var(--pixel-primary);
  transition: background-color 0.2s ease, transform 0.1s cubic-bezier(0.25, 0.46, 0.45, 0.94);
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
}

/* ????? */
.size-small {
  padding: 8px 16px;
  font-size: 14px;
}
.size-medium {
  padding: 12px 24px;
  font-size: 16px;
}
.size-large {
  padding: 16px 32px;
  font-size: 18px;
}

/* ??????? */
.my-button.active:not(.disabled) {
  background-color: var(--pixel-highlight) !important;
  color: var(--pixel-primary) !important;
}

.my-button:not(.disabled) {
  background-color: var(--pixel-primary) !important;
  color: var(--pixel-highlight);
}
.my-button.secondary:not(.disabled) {
  background-color: var(--pixel-secondary) !important;
}
.my-button.ghost:not(.disabled) {
  background-color: var(--pixel-background) !important;
  color: var(--pixel-ghost);
  border: 1px solid var(--pixel-ghost);
}

.my-button:not(.disabled):hover {
  background-color: var(--pixel-secondary);
}
.my-button:not(.disabled):active {
  transform: scale(1.05);
}

.my-button.disabled {
  background-color: var(--pixel-background);
  color: var(--pixel-disabled);
  cursor: not-allowed;
  opacity: 0.7;
}
.content-wrapper {
  display: inherit;
  align-items: inherit;
  gap: inherit;
}
</style>
