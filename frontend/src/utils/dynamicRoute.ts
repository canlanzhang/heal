import router from '@/router';

// 1. 预先获取 views 目录下所有的 .vue 文件模块
// 这里的 key 会是类似 '../views/home.vue' 的路径字符串
const modules = import.meta.glob('../views/**/*.vue');

export const generateRoutes = (menus: any[]) => {
  menus.forEach((menu) => {
    // 2. 构造与 glob 匹配的路径字符串
    // 注意：这里必须确保 menu.name 对应的文件名确实存在于 views 目录下
    const path = `../views/${menu.name}.vue`;

    // 3. 从 modules 对象中取出对应的加载函数
    const componentLoader = modules[path];

    if (componentLoader) {
      router.addRoute('layout', {
        path: menu.path,
        name: menu.name,
        // 直接使用预定义的加载函数
        component: componentLoader
      });
    } else {
      console.warn(`未找到组件: ${path}`);
    }
  });
};