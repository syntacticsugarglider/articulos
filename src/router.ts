import Vue from 'vue';
import Router from 'vue-router';
import Editor from './views/Editor.vue';

Vue.use(Router);

export default new Router({
  mode: 'history',
  base: process.env.BASE_URL,
  routes: [
    {
      path: '/editor',
      name: 'editor',
      component: () => import('./views/Editor.vue'),
    },
  ],
});
