const modules = import.meta.glob('@/views/**/index.vue')

export function buildDynamicRoutes(menus: any[]) {
  console.log('🔥 ALL MODULES:', Object.keys(modules))
  console.log('🔥 MENUS:', menus)

  return menus.map(menu => {
    const clean = menu.path.replace(/^\//, '')

    console.log('🧠 checking menu:', clean)

    const key = Object.keys(modules).find(k => {
      const ok = k.includes(clean)

      if (ok) {
        console.log('✅ MATCH FOUND:', k)
      }

      return ok
    })

    if (!key) {
      console.warn('❌ NO MATCH FOR:', menu.path)
    }

    return {
      name: menu.name,
      path: menu.path,
      component: key
        ? () => modules[key]().then(m => m.default)
        : () => import('@/views/error/404.vue')
    }
  })
}