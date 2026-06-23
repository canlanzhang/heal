import router from '@/router'
import { useUserStore } from '@/store/user'
import { buildDynamicRoutes } from '@/router/dynamic'

let inited = false

export async function bootstrap() {
  if (inited) return
  inited = true
console.log("1")
  const store = useUserStore()
console.log("2")
  const token = localStorage.getItem('token')
  if (!token) return
console.log("3")
  store.setToken(token)
console.log("4")
  await store.fetchProfile()
console.log("5")
  console.log('menus:', store.menus)

  const routes = buildDynamicRoutes(store.menus)

  console.log('routes:', routes)

  routes.forEach(r => {
    console.log('➕ add route:', r.path)

    router.addRoute('layout', r)
  })
}