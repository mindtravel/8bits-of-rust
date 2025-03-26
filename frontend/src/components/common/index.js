import MyRoll from './MyRoll.vue'
import MyButton from './MyButton.vue';
import MyInput from './MyInput.vue';
import MyText from './MyText.vue';
import MyKnob from './MyKnob.vue';
import MySlider from './MySlider.vue';
import MySelect from './MySelect.vue';
import MyGrid from './MyGrid.vue';
import MyLeftBar from './MyLeftBar.vue';

const components = {
  MyButton,
  MyInput,
  MyText,
  MyKnob,
  MySelect,
  MySlider,
  MyRoll,
  MyGrid,
  MyLeftBar,
};

export default {
    install(app) {
        Object.values(components).forEach((component) => {
            app.component(component.name, component);
        });
    }
}