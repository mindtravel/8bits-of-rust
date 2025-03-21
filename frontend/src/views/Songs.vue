<template>
    <div>
    <my-button 
      type="primary" 
      @click="addItem" 
      text="新增歌曲"
      class="add-btn"
      size = "medium"
    />    
    <songs-form v-model="name" ref="songsFormRef"/>
    <!-- <songs-list ref="songsListRef"/> -->
    </div>
</template>
<script>
import MyButton from "@/components/utils/MyButton.vue";
// import SongsList from "@/components/songs/SongsList.vue";
import SongsForm from '@/components/songs/SongsForm.vue';
export default {
    name: "Songs",
    components: { 
        MyButton,
        SongsForm,
    },
    mounted() {
        this.$nextTick(() => {
            console.log('songs已初始化:')
        })
    },
    methods: {
        addItem() {
            this.$refs.songsFormRef.addItem();
        }
    }
};
</script>
<style>
</style>