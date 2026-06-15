import { createRouter, createWebHistory } from 'vue-router';
import { useUserStore } from '@/store/user';
const routes = [
  {
    path: '/login',
    component: () => import('@/views/Login.vue')
  },
  {
    path: '/',
    component: () => import('@/layout/BasicLayout.vue'),
    children: [
      {
        path: 'home',
        name: 'home',
        component: () => import('@/views/Home.vue')
      }
    ]
  }
];

const router = createRouter({
  history: createWebHistory(),
  routes
});

router.beforeEach((to, from, next) => {
  const userStore = useUserStore();

  if (to.path !== '/login' && !userStore.token) {
    next('/login');
  } else {
    next();
  }
});

export default router;