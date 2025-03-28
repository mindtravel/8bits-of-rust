<template>
  <div class="pixel-input">
    <el-input
      v-model="inputValue"
      :placeholder="placeholder"
      @input="handleInput"
      @change="handleChange"
      class="pixel-style"
    >
    </el-input>
  </div>
</template>

<script>
export default { name: 'MyInput' }
</script>
<script setup>
import { ref } from 'vue'
const props = defineProps({
  modelValue: String,
  placeholder: {
    type: String,
    default: '请输入...',
  },
})

const emit = defineEmits(['update:modelValue', 'input', 'change'])

const inputValue = ref(props.modelValue)

const handleInput = (val) => {
  emit('update:modelValue', val)
  emit('input', val)
}

const handleChange = (val) => {
  emit('change', val)
}
</script>

<style lang="scss" scoped>
.pixel-input {
  --pixel-color: var(--pixel-text);
  --bg-color: var(--pixel-primary);

  :deep(.el-input__wrapper) {
    background: var(--bg-color);
    border: 2px solid var(--pixel-border);
    border-radius: 0;
    max-width: 200px;
    padding: 4px 8px;
    font-family: 'Zpix', monospace;
    font-size: 14px;
  }
}
</style>
