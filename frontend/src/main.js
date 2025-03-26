import { createApp } from 'vue'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import router from '@/router'
import App from './App.vue'
import store from '@/store/store';

import '@/assets/theme.css';
import Common from '@/components/common';

const app = createApp(App)
app.use(ElementPlus)
app.use(router)
app.use(store)
app.use(Common)
app.mount('#app')