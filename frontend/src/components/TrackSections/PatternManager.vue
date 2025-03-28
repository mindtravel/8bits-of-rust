<template>
  <div
    @mousedown.left="handleCanvasMouseDown"
    @mouseup="handleCanvasMouseUp"
    @mousemove="handleCanvasMouseMove"
  >
    <!-- <div 
		v-for="row in 88"
	>
		<div
			v-for="col in 5"
			class ="pattern"
			:style="patternStyle(row, 3*(col - 1), 2)"
			@mousedown.right="deletepattern(pattern.id)"
			@mousedown.left="startDragpattern(pattern, $event)"
		>
		</div>
	</div> -->
    <div
      v-for="pattern in patterns"
      class="pattern"
      :style="patternStyle(88 - pattern.pitch, pattern.starttime, 2)"
      @mousedown.right="deletePattern(pattern.id)"
      @mousedown.left="dragPattern(pattern, $event)"
    ></div>
    <my-grid :n_rows="5" :h_rows="50" ref="gridEl" />
    <div class="pattern-resize-handle"></div>
  </div>
</template>
<script>
export default {
  name: 'Score',
}
</script>
<script setup>
import { ref, computed, onMounted } from 'vue'
import { useStore, mapState } from 'vuex'

const store = useStore()
const patterns = computed(() => store.state.patterns)

// store.state.patterns)
const gridEl = ref(null)

// const patterns = computed(() => {
// 	let pattern;
// 	for (pattern in store.state.patterns) {
// 		console.log(pattern.pitch);
// 	}
// 	return store.state.patterns
// })

// 音符样式计算
const patternStyle = (row, col, duration) => ({
  left: `${col * 25}px`,
  top: `${row * 50 + 1}px`,
  width: `${duration * 25 - 3 - 1}px`,
  height: `${50 - 2}px`,
})

// 事件处理函数
let dragState = null
const selectedpatterns = ref(new Set())
const selectionBox = ref({ x1: 0, y1: 0, x2: 0, y2: 0 })

onMounted(() => {
  // console.log("patterns: ", patterns.grid[0][0] )
  // store.commit('initpatterns')
})

const handleCanvasMouseDown = (e) => {
  for (var pattern = 0; pattern < store.state.patterns.length; pattern++) {
    console.log(pattern.id)
  }
  // let x2 = e.clientX - rect.left
  // let y2 = e.clientY - rect.top
  if (e.ctrlKey || e.metaKey) return

  if (e.target.classList.contains('grid')) {
    // 初始化框选
    const rect = e.target.getBoundingClientRect()
    // let x1 = e.clientX - rect.left
    // let y1 = e.clientY - rect.top
    // selectionBox.value = {
    // 	x1: e.clientX - rect.left,
    // 	y1: e.clientY - rect.top,
    // 	x2: e.clientX - rect.left,
    // 	y2: e.clientY - rect.top
    // }
    // dragState = { type: 'select' }
    // selectedpatterns.value.clear()
    // 创建新音符逻辑
    // const rect = e.target.getBoundingClientRect()
    const x = e.clientX - rect.left
    const y = e.clientY - rect.top

    const newPattern = {
      id: Date.now(),
      starttime: Math.floor(x / 25),
      duration: 1,
      pitch: 88 - Math.floor(y / 50),
    }
    // console.log('handleCanvasMouseDown', store.state.patterns, newpattern )
    store.commit('addPattern', newPattern)
  }
}

const addPattern = (e) => {
  if (e.ctrlKey || e.metaKey) return
  // if (e.target.classList.contains('grid')) {
  // 初始化框选
  const rect = e.target.getBoundingClientRect()
  const x = e.clientX - rect.left
  const y = e.clientY - rect.top

  const newPattern = {
    id: Date.now(),
    starttime: Math.floor(x / 25),
    duration: 1,
    pitch: 88 - Math.floor(y / 50),
  }
  // console.log('handleCanvasMouseDown', store.state.patterns, newpattern )
  store.commit('addPattern', newPattern)
  // }
}

const deletePattern = (id) => {
  store.commit('deletePattern', id)
}

const dragPattern = (pattern, e) => {
  dragState = {
    type: 'move',
    patternId: pattern.id,
    offsetX: e.offsetX,
    startX: e.clientX,
    startY: e.clientY,
    originalPos: { ...pattern },
  }
}

// const handleCanvasMouseMove = (e) => {
// 	if (!dragState) return

// 	// 处理框选更新
// 	if (dragState.type === 'select') {
// 		const rect = e.target.getBoundingClientRect()
// 		selectionBox.value.x2 = e.clientX - rect.left
// 		selectionBox.value.y2 = e.clientY - rect.top

// 		// 计算选中范围
// 		store.state.patterns.forEach(pattern => {
// 			const patternRect = {
// 				left: pattern.start * 25,
// 				top: (88 - pattern.pitch) * 50,
// 				right: (pattern.start + pattern.duration) * 25,
// 				bottom: (88 - pattern.pitch + 1) * 50
// 			}

// 			if (isRectOverlap(selectionBox.value, patternRect)) {
// 				selectedpatterns.value.add(pattern.id)
// 			} else {
// 				selectedpatterns.value.delete(pattern.id)
// 			}
// 		})
// 	}

// 	// 处理拖拽移动逻辑
// 	const dx = e.clientX - dragState.startX
// 	const dy = e.clientY - dragState.startY

// 	if (dragState.type === 'move') {
// 		const newpattern = {
// 			...dragState.originalPos,
// 			start: Math.floor((dragState.originalPos.start * 25 + dx) / 25),
// 			row: 88 - Math.floor((dragState.originalPos.row * 50 + dy) / 50)
// 		}
// 		store.commit('updatepatternlength', newpattern)
// 	}
// }

// const handleCanvasMouseUp = () => {
// 	if (dragState?.type === 'select') {
// 		// 提交框选结果
// 		store.commit('updateSelection', Array.from(selectedpatterns.value))
// 	}
// 	dragState = null
// }

// // 矩形碰撞检测
// const isRectOverlap = (a, b) => {
// 	return a.x1 < b.right && a.x2 > b.left && a.y1 < b.bottom && a.y2 > b.top
// }

// // 键盘事件监听
// document.addEventListener('keydown', (e) => {
// 	if (e.key === 'Delete') {
// 		store.commit('DELETE_SELECTED')
// 	}
// 	if ((e.ctrlKey || e.metaKey) && e.key === 'c') {
// 		const patternsToCopy = store.state.patterns.filter(n => store.state.selectedpatterns.includes(n.id))
// 		store.commit('COPY_patternS', patternsToCopy)
// 	}
// })
</script>

<style scoped>
.pattern {
  background-color: rgb(255, 232, 172);
  z-index: 9;
  opacity: 1;
  position: absolute;
  border-right: 3px solid rgb(255, 191, 0);
}
</style>
