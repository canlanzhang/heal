
import { defineStore, storeToRefs } from 'pinia'
import { getProfileApi } from '@/api/auth'
import router from '@/router'
export const useUserStore = defineStore('user', {

  
  state: () => ({
    token: null,
    authReady: false,
    username: "",
    user: null,
    menus: [],
    routesInited: false,
    hasInitRoutes : false
    // : false
  }),

  actions: {
    setToken(token: string) {
      this.token = token
      localStorage.setItem('token', token)
    },
    setAuthReady(authReady: boolean) {
      console.log("setAuthReady: "+authReady)
      this.authReady = authReady
      localStorage.setItem('authReady', JSON.stringify(authReady))
    },

    setUserName(username: string) {
      this.username = username
      localStorage.setItem('username', username)
    },

    setMenus(menus: any[]) {
      this.menus = menus
    },

    logout() {
      // 1️⃣ 清 store
      this.token = ''
      this.user = null
      this.menus = []
      this.routesInited = false

      // 2️⃣ 清缓存
      localStorage.removeItem('token')

      // 3️⃣ ⭐关键：重置 router
      this.resetRouter()

      // 4️⃣ 跳转登录
      router.push('/login')
    },

    resetRouter() {
      // 保留静态路由，删除动态路由
      const routes = router.getRoutes()

      routes.forEach(r => {
        if (r.name && r.name !== 'login' && r.name !== 'layout' && r.name !== 'home') {
          router.removeRoute(r.name)
        }
      })
    },

    // ⭐关键：用 token 拉用户信息 + menus
    async fetchProfile() {
      const res = await getProfileApi()

      // 假设后端结构
      
      this.menus = res.data.menus
      this.user = res.data.user
      console.log("fetchProfile(): "+res)
      return res.data
    }
  }
})