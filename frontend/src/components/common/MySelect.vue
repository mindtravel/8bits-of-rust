<template>
  <div class="pixel-select" @click="toggleDropdown">
    <div class="selected-option">
      <my-text :content="selectedLabel || '点击选择...'" size="medium" />
      <div class="dropdown-arrow">▼</div>
    </div>
    <div v-show="isOpen" class="dropdown-options">
      <div
        v-for="(option, index) in options"
        :key="index"
        class="option"
        @click.stop="selectOption(option)"
      >
        <my-text :content="option.label" size="medium" />
      </div>
    </div>
  </div>
</template>

<script>
export default {
  props: {
    options: {
      type: Array,
      required: true,
      validator: (value) => value.every((opt) => 'value' in opt && 'label' in opt),
    },
    value: {
      type: [String, Number],
      default: '',
    },
  },
  data() {
    return {
      isOpen: false,
      selectedLabel: '',
    }
  },
  watch: {
    value: {
      immediate: true,
      handler(newVal) {
        const selected = this.options.find((opt) => opt.value === newVal)
        this.selectedLabel = selected ? selected.label : ''
      },
    },
  },
  methods: {
    toggleDropdown() {
      this.isOpen = !this.isOpen
    },
    selectOption(option) {
      this.$emit('input', option.value)
      this.selectedLabel = option.label
      this.isOpen = false
    },
  },
}
</script>

<style scoped>
.pixel-select {
  position: relative;
  max-width: 180px;
  cursor: pointer;
  background: var(--pixel-primary);
  border: 3px solid;
  border-color: var(--pixel-border);
  padding: 8px;
}

.my-select {
  position: relative;
  cursor: pointer;
}

.selected-option {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.dropdown-arrow {
  font-size: 12px;
  margin-left: 8px;
}

.dropdown-options {
  position: absolute;
  top: 100%;
  left: -3px;
  right: -3px;
  background: var(--pixel-primary);
  border: 3px solid var(--pixel-border);
  margin-top: 4px;
  z-index: 1;
}

.option {
  padding: 8px;
  &:hover {
    background: rgba(255, 255, 255, 0.1);
  }
}
</style>
