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
      },
      {
        path: 'admin',
        name: 'admin',
        component: () => import('@/views/AdminList.vue')
      },
      {
        path: 'article',
        name: 'article',
        component: () => import('@/views/article/List.vue')
      },
      {
        path: 'article/edit',
        name: 'article-create',
        component: () => import('@/views/article/Edit.vue')
      },
      {
        path: 'article/edit/:id',
        name: 'article-edit',
        component: () => import('@/views/article/Edit.vue')
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