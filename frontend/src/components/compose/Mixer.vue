<template>
<table>
    <tbody>
    <tr v-for="n in this.n_channels" :key="n">
        <td><my-text v-bind:content="'音轨'+ (n).toString() " size="large"/></td>
        <td><my-button text="合成器"/></td>
        <td><my-button text="效果器1"/></td>
        <td><my-button text="效果器2"/></td>
        <td><my-button text="效果器3"/></td>
        <td><my-text v-bind:content="'音量：'"/><my-knob class="label" :MaxVal="100" :val="80"/></td>
        <td><my-text v-bind:content="'声相：'"/><my-knob class="label" :MinVal="-100" :MaxVal="100" :val = "0" :ang = "30"/></td>
    </tr>
    </tbody>
</table>
</template>
<script>
import MyText from "@/components/utils/MyText.vue";
import MyButton from "@/components/utils/MyButton.vue";
import MyKnob from "@/components/utils/MyKnob.vue";
export default {
    name: 'Mixer',
    components: {
        'my-text': MyText,
        'my-button': MyButton,
        'my-knob': MyKnob,
    },
    data() {
        return {
            n_channels: 5,
        }
    },
}

</script>
<style>
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