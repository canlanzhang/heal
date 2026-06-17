import { createRouter, createWebHistory } from 'vue-router'
import { useUserStore } from '@/store/user'

const routes = [
  {
    path: '/login',
    name: 'login',
    component: () => import('@/views/login/index.vue')
  },
  {
    path: '/',
    name: 'layout',   // ⭐关键
    component: () => import('@/layout/BasicLayout.vue'),
    redirect: '/home',
    children: [
      {
        path: 'home',
        name: 'home',
        component: () => import('@/views/home/index.vue')
      }
      // ⚠️ admin / article 可以后端动态加
    ]
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

/**
 * 全局守卫
 */
router.beforeEach((to, from, next) => {
  const store = useUserStore()

  const token = store.token

  // ❌ 未登录
  if (to.path !== '/login' && !token) {
    next('/login')
    return
  }

  // ❌ 已登录不能回登录
  if (to.path === '/login' && token) {
    next('/home')
    return
  }

  next()
})

export default router