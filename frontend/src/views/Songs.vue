<template>
<div class="a">
    <my-button 
        type="primary" 
        @click="addItem" 
        text="新增歌曲"
        class="add-btn"
        size = "medium" />
    <songs-form v-model="newSongName" ref="songsFormRef" />
    <!-- <songs-list ref="songsListRef"/> -->
</div>
</template>
<script>

// import SongsList from "@/components/songs/SongsList.vue";
import { ref } from 'vue';
import SongsForm from '@/components/Songs/SongsForm.vue';
export default {
    components: { 
        SongsForm,
    },
    setup() {
        const newSongName = ref('');
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
.a {
    display: inline-block;
}
</style>