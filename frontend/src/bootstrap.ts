import router from '@/router'
import { useUserStore } from '@/store/user'
import { buildDynamicRoutes } from '@/router/dynamic'

export async function bootstrap() {
  const store = useUserStore()

  const token = store.token
  if (!token) return

  try {
    // ⭐没有 menus（刷新情况）
    if (!store.menus.length) {
      await store.fetchProfile()
    }

    // ⭐生成动态路由
    const routes = buildDynamicRoutes(store.menus)

    // ⭐避免重复 addRoute
    routes.forEach(r => {
      if (!router.hasRoute(r.name)) {
        router.addRoute('layout', r)
      }
    })

  } catch (e) {
    console.error('bootstrap error', e)
    store.logout()
    router.push('/login')
  }
}