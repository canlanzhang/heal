import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import { createPinia } from 'pinia'
import { bootstrap } from './bootstrap'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'

async function start() {
  const app = createApp(App)

  // 1. 优先注册 Pinia（确保路由守卫中可以使用 Store）
  app.use(createPinia())
  
  // 2. 注册路由
  app.use(router)

  try {
    // 3. ⭐ 关键：恢复动态路由
    await bootstrap()
  } catch (error) {
    console.error('应用初始化失败:', error)
    // 可以在这里处理初始化失败的逻辑，比如跳转到错误页或登录页
    // router.push('/login')
  }

  // 4. 注册 UI 组件库
  app.use(ElementPlus)

  // 5. 挂载应用
  app.mount('#app')
}

start()