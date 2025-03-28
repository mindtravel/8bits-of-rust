<template>
  <div class="pixel-radio" :class="{ disabled: disabled, checked: isChecked }" @click="toggleRadio">
    <div class="radio-box">
      <div class="radio-inner"></div>
    </div>
    <my-text :content="label" size="medium" />
  </div>
</template>

<script>
export default {
  name: 'MyRadio',
  model: {
    prop: 'value',
    event: 'input',
  },
  props: {
    value: {
      type: [String, Number],
      default: '',
    },
    label: {
      type: String,
      required: true,
    },
    disabled: {
      type: Boolean,
      default: false,
    },
    checkedValue: {
      type: [String, Number],
      required: true,
    },
  },
  computed: {
    isChecked() {
      return this.value === this.checkedValue
    },
  },
  methods: {
    toggleRadio() {
      if (!this.disabled) {
        this.$emit('input', this.checkedValue)
      }
    },
  },
}
</script>

<style scoped>
.pixel-radio {
  display: flex;
  align-items: center;
  cursor: pointer;
  padding: 8px 12px;
  margin: 4px;
  min-width: 100px;
  height: 32px;
  background: var(--pixel-background);
  border: 3px solid;
  border-image: repeating-linear-gradient(
      135deg,
      var(--pixel-primary) 0,
      var(--pixel-primary) 2px,
      transparent 2px,
      transparent 4px
    )
    3;
}

.radio-box {
  width: 20px;
  height: 20px;
  border: 3px solid var(--pixel-primary);
  margin-right: 12px;
  position: relative;
  clip-path: polygon(0 0, 100% 0, 100% calc(100% - 4px), calc(100% - 4px) 100%, 0 100%);
}

.radio-inner {
  width: 8px;
  height: 8px;
  background: var(--pixel-primary);
  position: absolute;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
  opacity: 0;
  transition: opacity 0.2s;
}

.checked .radio-inner {
  opacity: 1;
}

.disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.disabled .radio-box {
  border-image: repeating-linear-gradient(
      135deg,
      #666 0,
      #666 2px,
      transparent 2px,
      transparent 4px
    )
    2;
}
</style>
