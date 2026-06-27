import router from '@/router'
import { useUserStore } from '@/store/user'
import { buildDynamicRoutes } from '@/router/dynamic'

let bootstrapPromise: Promise<void> | null = null

export async function bootstrap() {
  if (bootstrapPromise) return bootstrapPromise

  bootstrapPromise =new Promise(async (resolve) => {
    const store = useUserStore()

    console.log('🔥 bootstrap START')

    // ❗防重复执行
    if (store.hasInitRoutes) {
      resolve()
      return
    }

    // ❗必须有菜单
    if (!store.menus || store.menus.length === 0) {
      console.warn('❌ no menus, skip bootstrap')
      resolve()  // ← 必须 resolve，否则调用方永远等待
      return
    }

    // 1️⃣ 生成路由
    const routes = buildDynamicRoutes(store.menus)

    console.log('📦 dynamic routes:', routes)

    // 2️⃣ 注册路由
    routes.forEach(r => {
      router.addRoute('layout', r)
    })

    // 3️⃣ 标记已初始化
    store.hasInitRoutes = true



    console.log('✅ bootstrap done')




    resolve()
  })
return bootstrapPromise
  
}