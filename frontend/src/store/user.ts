import { defineStore } from 'pinia'

export const useUserStore = defineStore('user', {
  state: () => ({
    token: '',
    user: null,
    menus: []
  }),

  actions: {
    setToken(token: string) {
      this.token = token
      localStorage.setItem('token', token)
    },

    loadToken() {
      this.token = localStorage.getItem('token') || ''
    },

    setUserInfo(user: any) {
      this.user = user
    },

    setMenus(menus: any[]) {
      this.menus = menus
    },

    clear() {
      this.token = ''
      this.user = null
      this.menus = []
      localStorage.removeItem('token')
    }
  }
})