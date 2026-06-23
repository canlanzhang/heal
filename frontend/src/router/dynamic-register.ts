import router from '@/router'

export function registerDynamicRoutes(routes: any[]) {
  routes.forEach(r => {
    if (!router.hasRoute(r.name)) {
      router.addRoute('layout', r)
      console.log('➕ route added:', r.name)
    }
  })
}