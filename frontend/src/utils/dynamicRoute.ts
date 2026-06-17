import router from '@/router'

export function generateRoutes(menus: any[]) {
  menus.forEach(menu => {
    const route = {
      path: menu.path.replace('/', ''),
      name: menu.name,
      component: () =>
        import(`@/views/${menu.name}/index.vue`)
    }

    if (!router.hasRoute(menu.name)) {
      router.addRoute('layout', route)
    }
  })
}