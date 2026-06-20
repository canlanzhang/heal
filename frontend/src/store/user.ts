import { defineStore } from 'pinia'
import { getProfileApi } from '@/api/auth'

export const useUserStore = defineStore('user', {
  state: () => ({
    token: localStorage.getItem('token') || '',
    // ⭐关键：持久化
    user: JSON.parse(localStorage.getItem('user') || '[]'),
    menus: JSON.parse(localStorage.getItem('menus') || '[]'),
    
  }),

  actions: {

    async fetchProfile() {
      const res = await getProfileApi()

      this.user = res.data.admin
      this.menus = res.data.menus

      // ⭐持久化
      localStorage.setItem('user', JSON.stringify(this.user))
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