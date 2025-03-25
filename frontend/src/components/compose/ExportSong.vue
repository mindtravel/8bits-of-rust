<template>
<my-text v-bind:content="'歌曲名字为：'+songName" size = "large"/><br>
<my-text v-bind:content="'导出格式为：'+format" size = "large"/><br>
<my-text v-bind:content="'预计占用空间为为：'+estimated_space+'MB'" size = "large"/>
<div class="placeholder-block"></div>
<my-button size="large" text="导出"/>
</template>
<script>
import MyText from "@/components/utils/MyText.vue";
import MyButton from "@/components/utils/MyButton.vue";
export default {
    name: 'PianoRoll',
    components: {
        'my-text': MyText,
        'my-button': MyButton,
    },
    data() {
        return {
            format: 'default',
            songName: 'default',
            estimated_space: 0,
        }
    },
}

</script>
<style>
.placeholder-block {
  width: 300px;       /* 指定宽度 */
  height: 100px;      /* 指定高度 */
}
</style>