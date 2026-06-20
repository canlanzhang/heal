import router from '@/router'
import { useUserStore } from '@/store/user'
import { buildDynamicRoutes } from '@/router/dynamic'

let inited = false

export async function bootstrap() {
  const store = useUserStore()

  if (inited) return
  inited = true

  const token = store.token || localStorage.getItem('token')
  if (!token) return

  store.token = token

  try {
    if (!store.user || !store.menus.length) {
      await store.fetchProfile()
    }

    console.log('menus:', store.menus)

    const routes = buildDynamicRoutes(store.menus)

    routes.forEach(r => {
      if (!router.hasRoute(r.name)) {
        router.addRoute('layout', r)
      }
    })

    // 🚨🔥 关键修复（你缺的就是这行）
    router.replace(router.currentRoute.value.fullPath)

  } catch (e) {
    console.error('bootstrap error', e)
    store.logout()
    router.push('/login')
  }
}