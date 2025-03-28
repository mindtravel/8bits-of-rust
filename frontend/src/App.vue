<template>
  <div class="card-container">
    <!-- <audio v-if="StartRoute" autoplay loop>
      <source src="@/assets/bgm.mp3" type="audio/mpeg">
      Your browser does not support the audio element.
    </audio> -->
    <router-link v-if="StartRoute" to="/compose">
      <my-button text="创作你的8bit音乐" />
    </router-link>
    <!-- <my-test></my-test> -->
    <div v-show="!StartRoute">
      <router-link to="/compose">
        <my-button text="创作" :active="$route.path === '/compose'" />
      </router-link>
      |
      <router-link to="/songs">
        <my-button text="歌曲" :active="$route.path === '/songs'" />
      </router-link>
      |
      <router-link to="/developers">
        <my-button text="开发者" :active="$route.path === '/developers'" />
      </router-link>
      <br />
      <br />
    </div>
    <router-view></router-view>
  </div>
</template>

<script>
import Compose from '@/views/Compose.vue'
import Songs from '@/views/Songs.vue'
import Developers from '@/views/Developers.vue'

console.log('begin')
export default {
  components: {
    Compose,
    Songs,
    Developers,
  },
  computed: {
    StartRoute() {
      console.log(this.$route.path === '/')
      return this.$route.path === '/'
    },
  },
  mounted() {
    this.$el.addEventListener('selectstart', this.handleSelectStart)
    this.$el.addEventListener('contextmenu', this.handleContextMenu)
  },
  beforeDestroy() {
    this.$el.removeEventListener('selectstart', this.handleSelectStart)
    this.$el.removeEventListener('contextmenu', this.handleContextMenu)
  },
  methods: {
    handleSelectStart(e) {
      if (!e.target.matches('input, textarea, [contenteditable="true"]')) {
        e.preventDefault()
      }
    },
    handleContextMenu(e) {
      if (!e.target.matches('input, textarea, [contenteditable="true"]')) {
        e.preventDefault()
      }
    },
  },
}
</script>

<style>
.card-container::before {
  content: '';
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: url('./assets/image.png') rgb(255, 157, 36) repeat;
  background-blend-mode: multiply;
  opacity: 0.3;
  z-index: -1;
}

.content {
  position: relative;
  z-index: 0; /* 确保内容在背景层上方 */
}
</style>
