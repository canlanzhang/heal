import router from '@/router'
import { useUserStore } from '@/store/user'
import { bootstrap } from '@/bootstrap'

console.log('🔥 guard loaded')

router.beforeEach(async (to, from, next) => {
  const store = useUserStore()
  const token = localStorage.getItem('token')

  console.log('🚦 guard:', to.path)
  console.log(store.menus.length);

  // ⭐关键1：如果有 token 且没初始化菜单 → 初始化
  if (token && !store.hasInitRoutes) {
    await bootstrap()
    console.log("if (token && !store.menus.length)里面");
  }
console.log("if (token && !store.menus.length)之后");
  // ⭐关键2：如果没有 token，只允许去 login
  if (!token && to.path !== '/login') {
    return next('/login')
  }

  // ⭐关键3：如果已经在 login，不要再 redirect login
  if (to.path === '/login') {
    return next()
  }

  next()
})