import router from '@/router'
import { useUserStore } from '@/store/user'
import { buildDynamicRoutes } from '@/router/dynamic'



export async function bootstrap() {
  const store = useUserStore()

  console.log('🔥 bootstrap START')

  const token = localStorage.getItem('token')
  if (!token) return

  store.setToken(token)

  await store.fetchProfile()

  console.log('🔥 menus:', store.menus)

  const routes = buildDynamicRoutes(store.menus)

  console.log('🔥 generated routes:', routes)   // ⭐ 就写这里

  console.log('🔥 routes:', routes)

  routes.forEach(r => {
    console.log('➕ add route:', r.path)

    router.addRoute('layout', r)
  })

  // ⭐关键修复
router.replace(router.currentRoute.value.fullPath)
  console.log('🔥 AFTER ROUTER:', router.getRoutes())
  store.hasInitRoutes = true
}