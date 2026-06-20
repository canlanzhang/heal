import { defineStore } from 'pinia'
import { getUserInfoApi } from '../api/auth'

export const useUserStore = defineStore('user', {
  state: () => ({
    token: localStorage.getItem('token') || '',
    user: null,

    // ⭐关键：持久化菜单
    menus: JSON.parse(localStorage.getItem('menus') || '[]'),
  }),

  actions: {
    setToken(token: string) {
      this.token = token
      localStorage.setItem('token', token)
    },

    async fetchProfile() {
      const res = await getUserInfoApi()
console.log(res);
      this.user = res.data.admin
      this.menus = res.data.menus

      // ⭐持久化
      localStorage.setItem('menus', JSON.stringify(this.menus))

      return res.data
    },

    logout() {
      this.token = ''
      this.user = null
      this.menus = []

      localStorage.removeItem('token')
      localStorage.removeItem('menus')

      // ⭐关键：刷新 router
      location.href = '/login'

    }
  }
})