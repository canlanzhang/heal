import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import { createPinia } from 'pinia'
import { bootstrap } from './bootstrap'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'

async function start() {
  const app = createApp(App)

  app.use(createPinia())
  app.use(router)

  // ⭐关键：恢复动态路由
  await bootstrap()

  app.use(ElementPlus)

  app.mount('#app')

}



start()




