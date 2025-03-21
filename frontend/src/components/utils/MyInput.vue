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

<script setup>
  import { ref, defineProps, defineEmits } from 'vue'
  
  const props = defineProps({
    modelValue: String,
    placeholder: {
      type: String,
      default: '请输入...'
    }
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
  --pixel-color: #333;
  --bg-color: #f0f0f0;

  :deep(.el-input__wrapper) {
    background: var(--bg-color);
    // border: 2px solid var(--pixel-color);
    // border-radius: 0;
    // box-shadow: 3px 3px 0 var(--pixel-color);
    padding: 4px 8px;
    font-family: 'Zpix', monospace;
    font-size: 14px;
    
    &:hover {
      // box-shadow: 4px 4px 0 var(--pixel-color);
    }
    
    &.is-focus {
      // box-shadow: 2px 2px 0 var(--pixel-color);
      border-color: #ff6b6b;
    }
  }
}
</style>