<template>
<div @mousedown="handleCanvasMouseDown" @mouseup="handleCanvasMouseUp" @mousemove="handleCanvasMouseMove">
	<!-- <div 
		v-for="note in notes"
		:key="note.id"
		class="note"
		:style="noteStyle(note)"
		@mousedown.right="deleteNote(note.id)"
		@mousedown.left="startDragNote(note, $event)"
	></div> -->
	<my-grid 
		:n_rows="88" 
		:h_rows="20"
		ref="gridEl"
	/>

		<div class="note-resize-handle"></div>
</div>
</template>
<script>
export default {
	name: "Score",
}
</script>
<script setup>
import { ref, computed, onMounted} from 'vue'
import { useStore } from 'vuex'

const store = useStore()


// computed(() => store.state.notes)
const gridEl = ref(null)
// const notes = computed(() => {
// 	let note;
// 	for (note in store.state.notes) {
// 		console.log(note.pitch);
// 	}
// 	return store.state.notes
// })

// 音符样式计算
const noteStyle = (note) => ({
	left: `${note.start * 25}px`,
	top: `${(88 - note.pitch) * 20}px`,
	width: `${note.duration * 25}px`,
	height: '20px'
})

// 事件处理函数
let dragState = null
const selectedNotes = ref(new Set())
const selectionBox = ref({ x1: 0, y1: 0, x2: 0, y2: 0 })

// onMounted(() => {
// 	const notes = [
// 		{id:0, start:1, duration:10, pitch:2},   
// 		{id:1, start:1, duration:2, pitch:48},
// 	]
//   console.log('notes已初始化:', notes.value[0])
// })

// const handleCanvasMouseDown = (e) => {
// 	if (e.ctrlKey || e.metaKey) return

// 	if (e.target.classList.contains('grid')) {
// 		// 初始化框选
// 		const rect = e.target.getBoundingClientRect()
// 		selectionBox.value = {
// 			x1: e.clientX - rect.left,
// 			y1: e.clientY - rect.top,
// 			x2: e.clientX - rect.left,
// 			y2: e.clientY - rect.top
// 		}
// 		dragState = { type: 'select' }
// 		selectedNotes.value.clear()
// 		// 创建新音符逻辑
// 		// const rect = e.target.getBoundingClientRect()
// 		const x = e.clientX - rect.left
// 		const y = e.clientY - rect.top
		
// 		const newNote = {
// 			id: Date.now(),
// 			start: Math.floor(x / 25),
// 			duration: 1,
// 			row: 88 - Math.floor(y / 20)
// 		}
// 		store.commit('addNote', newNote)
// 	}
// }

// const deleteNote = (id) => {
// 	store.commit('deleteNote', id)
// }

// const startDragNote = (note, e) => {
// 	dragState = {
// 		type: 'move',
// 		noteId: note.id,
// 		offsetX: e.offsetX,
// 		startX: e.clientX,
// 		startY: e.clientY,
// 		originalPos: { ...note }
// 	}
// }

// const handleCanvasMouseMove = (e) => {
// 	if (!dragState) return

// 	// 处理框选更新
// 	if (dragState.type === 'select') {
// 		const rect = e.target.getBoundingClientRect()
// 		selectionBox.value.x2 = e.clientX - rect.left
// 		selectionBox.value.y2 = e.clientY - rect.top
		
// 		// 计算选中范围
// 		store.state.notes.forEach(note => {
// 			const noteRect = {
// 				left: note.start * 25,
// 				top: (88 - note.pitch) * 20,
// 				right: (note.start + note.duration) * 25,
// 				bottom: (88 - note.pitch + 1) * 20
// 			}
			
// 			if (isRectOverlap(selectionBox.value, noteRect)) {
// 				selectedNotes.value.add(note.id)
// 			} else {
// 				selectedNotes.value.delete(note.id)
// 			}
// 		})
// 	}

// 	// 处理拖拽移动逻辑
// 	const dx = e.clientX - dragState.startX
// 	const dy = e.clientY - dragState.startY

// 	if (dragState.type === 'move') {
// 		const newNote = {
// 			...dragState.originalPos,
// 			start: Math.floor((dragState.originalPos.start * 25 + dx) / 25),
// 			row: 88 - Math.floor((dragState.originalPos.row * 20 + dy) / 20)
// 		}
// 		store.commit('updateNotelength', newNote)
// 	}
// }

// const handleCanvasMouseUp = () => {
// 	if (dragState?.type === 'select') {
// 		// 提交框选结果
// 		store.commit('updateSelection', Array.from(selectedNotes.value))
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
// 		const notesToCopy = store.state.notes.filter(n => store.state.selectedNotes.includes(n.id))
// 		store.commit('COPY_NOTES', notesToCopy)
// 	}
// })
</script>

<style scoped>
.note {
	background-color: aquamarine;
	z-index: 10;
	position: absolute;
}
</style>