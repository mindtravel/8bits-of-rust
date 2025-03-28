<template>
  <div class="main-container">
    <table>
      <tbody>
        <tr v-for="n in this.n_channels">
          <td><my-text v-bind:content="'音轨' + n.toString()" size="large" /></td>
          <td><my-button text="合成器" /></td>
          <td><my-button text="效果器1" /></td>
          <td><my-button text="效果器2" /></td>
          <td><my-button text="效果器3" /></td>
          <td>
            <my-text v-bind:content="'音量：'" />
            <div class="vertical-slider">
              <my-text :content="volumes[n - 1] + '%'" class="volume-value" />
              <my-slider orientation="vertical" :min="0" :max="100" v-model="volumes[n - 1]" />
            </div>
          </td>
          <td>
            <my-text v-bind:content="'声相：'" />
            <my-knob
              class="label"
              v-model="panValues[n - 1]"
              :minVal="-100"
              :maxVal="100"
              :val="0"
            />
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>
<script>
export default {
  components: {},
  data() {
    return {
      n_channels: 5,
      volumes: [80, 80, 80, 80, 80],
      panValues: [0, 0, 0, 0, 0],
    }
  },
}
</script>
<style>
.vertical-slider {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin: 8px 0;
}

/* 保持原有表格样式不变 */
</style>
