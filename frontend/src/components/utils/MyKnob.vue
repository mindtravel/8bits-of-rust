<template>
    <div class="round_box" @wheel="handleWheel">
        <div class="round_right" :style="{ transform: 'rotate(' + ang + 'deg)' }"></div>
        <div class="round_num"><my-text v-bind:content="String(computedValue)" /></div>
    </div>
</template>

<script>
import MyText from './MyText.vue';

export default {
    name: 'MyKnob',
    components: {
        'my-text': MyText,
    },
    props: {
        MaxVal: {
            type: Number,
            default: 1000,
        },
        MinVal: {
            type: Number,
            default: 0,
        },
        MinAng: {
            type: Number,
            default: -60,
        },
        MaxAng: {
            type: Number,
            default: 150,
        },
        value: {
            type: Number,
            default: 0,
        },
    },
    data() {
        return {
            ang: this.initialAngle, // 初始化角度
        };
    },
    computed: {
        // 根据传入的 value 动态计算初始角度
        initialAngle() {
            return (
                this.MinAng +
                ((this.value - this.MinVal) / (this.MaxVal - this.MinVal)) * (this.MaxAng - this.MinAng)
            );
        },
        // 根据内部 ang 动态计算当前值
        computedValue() {
            return Math.min(
                Math.max(
                    Math.round(
                        ((this.ang - this.MinAng) / (this.MaxAng - this.MinAng)) * (this.MaxVal - this.MinVal) +
                        this.MinVal
                    ),
                    this.MinVal
                ),
                this.MaxVal
            );
        },
    },
    watch: {
        // 当外部 `value` 属性改变时，更新内部角度
        value(newVal) {
            this.ang = this.initialAngle;
        },
    },
    mounted() {
        this.ang = this.initialAngle; // 初始化角度
    },
    methods: {
        handleWheel(event) {
            const delta = Math.sign(event.deltaY) * 10; // 调整滚轮敏感度
            const newAngle = this.ang + delta;
            if (newAngle >= this.MinAng && newAngle <= this.MaxAng) {
                this.ang = newAngle;
                const newVal = this.computedValue;
                this.$emit('input', newVal); // 实时更新父组件的值
            }
        },
    },
};
</script>

<style scoped>
.round_box {
  position: relative;
  width: 30px;
  height: 30px;
  border-radius: 50%;
  border: 10px solid #e6e6e6;
  background: #cacaca;
  cursor: pointer; /* 鼠标指针样式 */
}

.round_right {
  position: absolute;
  width: 20%;
  height: 20%;
  background: #6a6a6a;
  border-radius: 50%;
  transform-origin: 250% 250%;
  transition: transform 0.1s;
}

.round_num {
  position: absolute;
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 16px;
  color: #333;
}
</style>
