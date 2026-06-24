import router from '@/router'
import { useUserStore } from '@/store/user'
import { bootstrap } from '@/bootstrap'

console.log('🔥 guard loaded')

router.beforeEach(async (to, from, next) => {
  const store = useUserStore()
  const token = localStorage.getItem('token')

  console.log('🚦 guard:', to.path)

  // ❌ 1. 未登录直接去 login
  console.log("token: "+ token)

  if (!token && to.path !== '/login') {
    return next('/login')
  }


  // ✔ 2. 已登录访问 login，放行
  if (to.path === '/login') {
    return next()
  }

  // ⭐ 3. 初始化系统（只执行一次）
  
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

  next()
})