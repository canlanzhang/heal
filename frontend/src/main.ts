import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import { createPinia, storeToRefs } from 'pinia'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'

import '@/router/guard'
import { bootstrap } from '@/bootstrap'
import { loadRouteLocation } from 'vue-router'
import { useUserStore } from '@/store/user'
async function start() {
  const app = createApp(App)

  const pinia = createPinia()
  

  app.use(pinia)

  const store = useUserStore(pinia)
  const token = localStorage.getItem('token')
  const authReady = localStorage.getItem('authReady') === 'true'

  app.use(router)

    if(token){
    store.setToken(token)
    store.setAuthReady(authReady)
    //await store.fetchProfile()
    //await bootstrap()
    console.log("mm:  "+store.menus)
  }

  app.use(ElementPlus)

  // ⭐关键：必须在 mount 前完成初始化
  

  app.mount('#app')
}

start()