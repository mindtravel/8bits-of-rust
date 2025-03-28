import MyButton from './MyButton.vue';
import MyInput from './MyInput.vue';
import MyText from './MyText.vue';
import MyKnob from './MyKnob.vue';
import MySlider from './MySlider.vue';
import MySelect from './MySelect.vue';
import MyGrid from './MyGrid.vue';
import MyTest from './MyTest.vue';

const components = {
  MyButton,
  MyInput,
  MyText,
  MyKnob,
  MySelect,
  MySlider,
  MyGrid,
  MyTest,
};

export default {
    install(app) {
        Object.values(components).forEach((component) => {
            app.component(component.name, component);
        });
    }
}