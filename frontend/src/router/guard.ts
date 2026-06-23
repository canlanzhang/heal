import router from '@/router'
import { useUserStore } from '@/store/user'
import { bootstrap } from '@/bootstrap'

router.beforeEach(async (to, from, next) => {
  const store = useUserStore()

  console.log('🚦 guard:', to.path)

  if (!store.token && to.path !== '/login') {
    return next('/login')
  }

  // 🔥 兜底恢复（刷新或异常情况）
  if (!store.routesInited) {
    await bootstrap()
    return next({ ...to, replace: true })
  }

  next()
})