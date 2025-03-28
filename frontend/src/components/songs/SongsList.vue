<template>
  <div class="main-container">
    <table>
      <thead>
        <tr>
          <td><my-text content="序号" /></td>
          <td><my-text content="歌曲名称" /></td>
          <td><my-text content="创建时间" /></td>
          <td><my-text content="编辑" /></td>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(song, index) in storeSongs">
          <td><my-text :content="(index + 1).toString()" /></td>
          <td><my-text :content="song.name" /></td>
          <td><my-text :content="song.date" /></td>
          <td>
            <my-button text="删除" @click="deleteItem(index)" />
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script>
export default {}
</script>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useStore } from 'vuex'

// 定义 Props [3,5](@ref)
const props = defineProps({
  max_song_num: {
    type: Number,
    default: 10,
  },
})

const store = useStore()
const storeSongs = computed(() => store.state.songs)

// 时间格式化方法
const getCurrentTime = () => {
  const now = new Date()
  return now.toISOString().slice(0, 16).replace('T', ' ')
}

// 添加歌曲方法
const addItem = (name) => {
  store.commit('addSong', {
    name: name,
    date: getCurrentTime(),
  })
}

// 删除歌曲方法
const deleteItem = (index) => {
  if (confirm('确定删除这首歌曲吗？')) {
    store.commit('deleteSong', index)
  }
}

// 生命周期钩子 [4](@ref)
onMounted(() => {
  console.log('songsList已初始化:', storeSongs.value)
})

defineExpose({
  addItem,
  deleteItem,
})
</script>

<style scoped>
/* 保持原有样式 */
</style>
