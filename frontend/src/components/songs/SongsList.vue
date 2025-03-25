<template>
  <div>
    <table>
      <thead>
        <tr>
          <td><my-text content="序号"/></td>
          <td><my-text content="歌曲名称"/></td>
          <td><my-text content="创建时间"/></td>
          <td><my-text content="编辑"/></td>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(song, index) in songs" :key="index">
          <td><my-text v-bind:content="(index + 1).toString()"/></td>
          <td><my-text v-bind:content="song.name"/></td>
          <td><my-text v-bind:content="song.date"/></td>
          <td><my-button text="删除"
            @click = "deleteItem(index)"/></td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script>
import MyText from "@/components/utils/MyText.vue";
import MyButton from "@/components/utils/MyButton.vue";
export default {
  name: "SongsList",
  data() {
    return {
      songs: [], // 歌曲列表
    };
  },
  mounted() {
      this.$nextTick(() => {
          console.log('songsList已初始化:', this.songs)
      })
      this.songs = this.$store.state.songs;
  },
  components: {
    "my-text": MyText,
    "my-button": MyButton,
  },
  props: {
    max_song_num: {
      type: Number, // 指定类型为数字
      default: 10 // 默认值为10
    },
  },


  methods: {
    getCurrentTime() {
      const now = new Date();
      const year = now.getFullYear();
      const month = (now.getMonth() + 1).toString().padStart(2, '0'); // 补零
      const day = now.getDate().toString().padStart(2, '0');
      const hours = now.getHours().toString().padStart(2, '0');
      const minutes = now.getMinutes().toString().padStart(2, '0');
      return `${year}-${month}-${day} ${hours}:${minutes}`;
    },
    addItem(name) {
      this.$store.commit('addSong', { name: name, date: this.getCurrentTime() });
      console.log("stored songs", this.$store.state.songs);
    },
    // 在 Vue 组件的 methods 或 mounted 中

  //   editItem(index) {
  //     const newName = prompt('请输入新的歌曲名称', this.songs[index].name);
  //     if (newName) {
  //       this.songs[index].name = newName;
  //       this.saveSongs();
  //     }
  //   },
    deleteItem(index) {
      if (confirm('确定删除这首歌曲吗？')) {
        this.$store.commit('deleteSong', index);
      }
    },
  }
};
</script>
<style scoped>
table {
  width: 100%;
  border-collapse: collapse;
}
th, td {
  border: 1px solid #ddd;
  padding: 8px;
  text-align: left;
}
td {
  background-color: #ffffff;
}
</style>