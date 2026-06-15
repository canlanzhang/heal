import router from '@/router';

export const generateRoutes = (menus: any[]) => {
  menus.forEach((menu) => {
    router.addRoute('layout', {
      path: menu.path,
      name: menu.name,
      component: () => import(`@/views/${menu.name}.vue`)
    });
  });
};