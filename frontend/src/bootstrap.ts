import router from '@/router'
import { useUserStore } from '@/store/user'
import { buildDynamicRoutes } from '@/router/dynamic'

let inited = false

export async function bootstrap() {
  if (inited) return
  inited = true

  const store = useUserStore()

  const token = localStorage.getItem('token')
  if (!token) return

  store.setToken(token)

  await store.fetchProfile()

  console.log('menus:', store.menus)

  const routes = buildDynamicRoutes(store.menus)

  console.log('routes:', routes)

  routes.forEach(r => {
    console.log('➕ add route:', r.path)

    router.addRoute('layout', r)
  })
}