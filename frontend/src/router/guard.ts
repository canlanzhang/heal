import router from '@/router'
import { useUserStore } from '@/store/user'
import { bootstrap } from '@/bootstrap'
import { ro } from 'element-plus/es/locales.mjs'

console.log('🔥 guard loaded')

router.beforeEach(async (to, from, next) => {
  console.log('🔥 beforeEach')
  const store = useUserStore()
  const token = store.token || localStorage.getItem('token')

  console.log('🚦 guard to.path:', to.path)

  // ❌ 1. 未登录直接去 login
  if (!store.authReady && to.path != '/login') {
    console.log("未登录")
    //console.log(store.authReady)
    //console.log(to.path)
    return next('/login')
  }
  

  if(token && !store.routesInited)
  {
    console.log("已登录")
    console.log(router.options.routes) // 查看原始路由配置
console.log('🔥 ALL ROUTERS:', router.getRoutes())
    await store.fetchProfile()
    await bootstrap()
    store.routesInited = true

    



    console.log("已登录routesInited："+store.routesInited);
    return next({ ...to, replace: true })  // ← 不要写死 '/dashboard'，回到用户原来要去的页面
    //return next('/dashboard')
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

  next()
})