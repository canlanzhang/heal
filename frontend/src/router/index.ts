import { createRouter, createWebHistory } from 'vue-router'
// 引入刚才创建的登录组件
import Login from '../components/Login.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/', // 访问根目录时
      name: 'login',
      component: Login // 显示 Login 组件
    }
  ]
})

export default router