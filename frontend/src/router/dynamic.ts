const modules = import.meta.glob('@/views/**/index.vue')

const getComponent = (menuPath: string) => {
  const clean = menuPath.replace(/^\//, '')

  const key = Object.keys(modules).find(k =>
    k.includes(`/${clean}/index.vue`)
  )

  if (!key) {
    console.warn('❌ view not found:', menuPath)
    return null
  }

  return modules[key]
}

export function buildDynamicRoutes(menus: any[]) {
  console.log('🔥 buildDynamicRoutes')

  return menus.map(menu => {
    const comp = getComponent(menu.path)

    return {
      name: menu.name || menu.path.replace(/\//g, '_'),

      // ⭐关键：必须保持 /admin 这种完整路径
      path: menu.path,

      meta: menu.meta || {},

      component: comp
        ? () => comp().then(m => m.default)
        : () => import('@/views/error/404.vue')
    }
  })
}