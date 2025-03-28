<template>
  <div class="grid-container">
    <div v-for="(row, rowIndex) in grid" :key="rowIndex" class="row">
      <div
        v-for="(cell, colIndex) in row"
        :key="colIndex"
        class="cell"
        :class="{ active: cell.active }"
        @click="toggleCell(rowIndex, colIndex)"
      ></div>
    </div>
  </div>
</template>
<script>
export default {
  name: 'MyTest',
}
</script>
<script setup>
import { ref } from 'vue'

// 初始化 5x5 网格
const grid = ref(
  Array.from({ length: 5 }, () => Array.from({ length: 5 }, () => ({ active: false })))
)

// 切换格子状态
const toggleCell = (row, col) => {
  grid.value[row][col].active = !grid.value[row][col].active
}
</script>

<style scoped>
.grid-container {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.row {
  display: flex;
  gap: 2px;
}

.cell {
  width: 50px;
  height: 50px;
  background-color: #f0f0f0;
  border: 1px solid #ddd;
  cursor: pointer;
  transition: background-color 0.3s;
}

.cell.active {
  background-color: #42b983; /* 激活颜色 */
}
</style>
