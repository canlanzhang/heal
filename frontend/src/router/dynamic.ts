const modules = import.meta.glob('@/views/**/index.vue')

/**
 * menus → routes
 */
export function buildDynamicRoutes(menus: any[]) {
  return menus.map(menu => {
    return {
      path: menu.path.replace(/^\//, ''), // /menu -> menu
      name: menu.name,
      meta: {
        title: menu.title,
        icon: menu.icon,
      },
      component: modules[`/src/views${menu.path}/index.vue`],
    }
  })
}