import router from '@/router'
import { useUserStore } from '@/store/user'
import { bootstrap } from '@/bootstrap'
import { ro } from 'element-plus/es/locales.mjs'

console.log('🔥 guard loaded')

router.beforeEach(async (to, from, next) => {
  
  const store = useUserStore()
  const token = store.token || localStorage.getItem('token')
console.log("toekn:  "+ token)
  console.log('🚦 guard:', to.path)

  // ❌ 1. 未登录直接去 login
  if (!store.authReady && to.path != '/login') {
    console.log("未登录")
    console.log(store.authReady)
    console.log(to.path)
    console.log("去login登录")
    return next('/login')
  }
  
  console.log("toekn:  "+ token)
  if(token && !store.routesInited)
  {
    console.log("已登录")
    await store.fetchProfile()
    await bootstrap()
    store.routesInited = true
    return next('/dashboard')
  }

  


  // ✔ 2. 已登录访问 login，放行
  /*
  if (to.path === '/login') {
    console.log("已登录")
    store.setToken(token)
    await store.fetchProfile()   // ⭐只在这里

    await bootstrap()
  }
*/
  // ⭐ 3. 初始化系统（只执行一次）
  /*
  console.log("hasInitRoutes: "+ store.hasInitRoutes)
  if (token && !store.hasInitRoutes) {
    console.log("if (token && !store.hasInitRoutes) ")
    try {
      // ✔ 确保 menus 存在
      if (!store.menus || store.menus.length === 0) {
        await store.fetchProfile()
      }

      await bootstrap()
    } catch (e) {
      console.error('bootstrap error', e)
      return next('/login')
    }
  }
*/
  console.log("next() ")
  next()
})