import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import { createPinia } from 'pinia'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'

import '@/router/guard'
import { bootstrap } from '@/bootstrap'

async function start() {
  const app = createApp(App)

  const pinia = createPinia()

  app.use(pinia)
  app.use(router)
  app.use(ElementPlus)

  // ⭐关键：必须在 mount 前完成初始化
  await bootstrap()

  app.mount('#app')
}

start()