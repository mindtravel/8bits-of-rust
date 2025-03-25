<template>
<div>
    <tbody>
    <tr v-for="n in this.n_channels" :key="n">
        <td><my-text v-bind:content="'音轨'+ (n).toString() " size="large"/></td>
    </tr>
    </tbody>
</div>
</template>
<script>
import MyText from "@/components/utils/MyText.vue";
import MyButton from "@/components/utils/MyButton.vue";
export default {
    name: 'Arangement',
    components: {
        'my-text': MyText,
        'my-button': MyButton,
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