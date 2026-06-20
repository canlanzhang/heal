const modules = import.meta.glob('/src/views/**/index.vue')

export function buildDynamicRoutes(menus: any[]) {
  return menus.map(menu => {
    const cleanPath = menu.path.replace(/^\//, '')

    const matchKey = Object.keys(modules).find(key =>
      key === `/src/views/${cleanPath}/index.vue`
    )

    console.log('menu:', menu.path)
    console.log('matchKey:', matchKey)

    return {
      path: cleanPath, // ❗不能带 /
      name: menu.name,

      component: matchKey
        ? modules[matchKey]   // ❗关键：这里是函数
        : undefined,

      meta: {
        title: menu.title
      }
    }
  })
}