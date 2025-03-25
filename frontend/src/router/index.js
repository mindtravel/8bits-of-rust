import { createRouter, createWebHistory } from 'vue-router';
import Compose from '../views/Compose.vue';
import Songs from '../views/Songs.vue';
import Developers from '../views/Developers.vue';

const routes = [
  // {
  //   path: '/',
  //   name: 'Home',
  //   Component: Compose,
  // },
  {
    path: '/compose',
    name: 'Compose',
    component: Compose
  },
  {
    path: '/songs',
    name: 'Songs',
    component: Songs
  },
  {
    path: '/developers',
    name: 'Developers',
    component: Developers
  }
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
