<template>
  <div class="form-container">
    
    <el-form ref="form" :model="formItems" :rules="rules" inline>
      <div 
        v-for="(item, index) in formItems" 
        :key="index"
        class="form-row"
      >
        <el-form-item 
          :prop="`formItems[${index}].name`"
          :rules="rules.name"
        >
          <my-button class="item-btn">
            {{ item.name || '未命名歌曲' }}
          </my-button>
        </el-form-item>

        <my-button
          type="danger"
          @click="removeItem(index)"
          :disabled="formItems.length <= 1"
          content="删除"
          class="delete-btn"
        />
      </div>
    </el-form>
  </div>
</template>

<script>
import MyButton from './MyButton.vue'

export default {
  name: 'List',
  components: {
    MyButton
  },
  props:{
    formItems: {
      type: Array,
      default: () => []
    }
  }
}
</script>

<style scoped>
.form-container {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
}

.form-row {
  display: flex;
  align-items: center;
  margin-bottom: 12px;
  gap: 8px;
}

.item-btn {
  min-width: 120px;
  text-align: center;
  font-family: 'Zpix', monospace;
}

.delete-btn {
  margin-left: 8px;
}

.add-btn {
  margin-bottom: 20px;
}

/* 表单验证提示样式 */
:deep(.el-form-item__error) {
  position: absolute;
  top: 100%;
  left: 50%;
  transform: translateX(-50%);
  white-space: nowrap;
  font-size: 12px;
}
</style>