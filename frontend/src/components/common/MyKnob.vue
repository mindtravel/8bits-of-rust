<template>
  <div
    class="round_box"
    @wheel="handleWheel"
    @mousedown="handleMouseDown"
    @mousemove="handleMouseMove"
    @mouseup="handleMouseUp"
    @mouseleave="handleMouseUp"
    @dblclick="handleDoubleClick"
  >
    <div class="round_right" :style="{ transform: `rotate(${ang}deg)` }"></div>
    <div class="round_num">
      <my-text :content="String(modelValue)" />
    </div>
  </div>
</template>
<script>
export default { name: 'MyKnob' }
</script>
<script setup>
import { ref, computed, watch, onMounted } from 'vue'

const props = defineProps({
  modelValue: {
    type: Number,
    default: 0,
    required: true,
  },
  minVal: {
    type: Number,
    default: 0,
  },
  maxVal: {
    type: Number,
    default: 1000,
  },
  minAng: {
    type: Number,
    default: -90,
  },
  maxAng: {
    type: Number,
    default: 175,
  },
})

const emit = defineEmits(['update:modelValue'])

const ang = ref(0)
const angleRange = computed(() => props.maxAng - props.minAng)
const valueRange = computed(() => props.maxVal - props.minVal)

const initializeAngle = () => {
  ang.value =
    props.minAng + ((props.modelValue - props.minVal) / valueRange.value) * angleRange.value
  // console.log(props.minVal, props.maxVal, props.minAng, props.maxAng, props.modelValue)
}

onMounted(initializeAngle)

watch(
  () => props.modelValue,
  (newVal) => {
    const clampedValue = Math.max(props.minVal, Math.min(props.maxVal, newVal))

    if (clampedValue !== newVal) {
      emit('update:modelValue', clampedValue)
      return
    }
    initializeAngle()
  }
)

// 鼠标事件处理
const isDragging = ref(false)
const startX = ref(0)
const startValue = ref(0)

const handleMouseDown = (event) => {
  isDragging.value = true
  startX.value = event.clientX
  startValue.value = props.modelValue
  document.body.style.userSelect = 'none'
}

const handleMouseMove = (event) => {
  if (!isDragging.value) return

  const deltaX = event.clientX - startX.value
  const sensitivity = (props.maxVal - props.minVal) / 30
  const newVal = startValue.value + deltaX * sensitivity
  const clampedValue = Math.max(props.minVal, Math.min(props.maxVal, newVal))
  // 将限制后的数值转换为对应角度
  ang.value = props.minAng + ((clampedValue - props.minVal) / valueRange.value) * angleRange.value

  emit('update:modelValue', Math.round(clampedValue))
}

const handleMouseUp = () => {
  isDragging.value = false
  document.body.style.userSelect = ''
}

// 滚轮事件处理
const handleWheel = (event) => {
  event.preventDefault()
  const delta = Math.sign(event.deltaY) * 5
  const newAngle = ang.value - delta
  if (newAngle < props.minAng) {
    ang.value = props.minAng
  } else if (newAngle > props.maxAng) {
    ang.value = props.maxAng
  } else {
    ang.value = newAngle
  }
  const newValue = props.minVal + ((ang.value - props.minAng) / angleRange.value) * valueRange.value
  emit('update:modelValue', Math.round(newValue))
}

const handleDoubleClick = () => {
  const defaultValue = 0
  emit('update:modelValue', defaultValue)
}
</script>

<style scoped>
/* 原有样式保持不变 */
.round_box {
  position: relative;
  width: 50px;
  height: 50px;
  border-radius: 50%;
  border: 10px solid var(--pixel-border);
  background: var(--pixel-background) !important;
  cursor: pointer;
  .round_right {
    position: absolute;
    width: 20%;
    height: 20%;
    background: var(--pixel-primary) !important;
    border-radius: 50%;
    transform-origin: 250% 250%;
    transition: transform 0.1s;
  }
  .round_num {
    left: 1px;
    position: relative;
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 16px;
  }
}
</style>
