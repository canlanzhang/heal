import { defineStore } from 'pinia';

export const useUserStore = defineStore('user', {
  state: () => ({
    token: localStorage.getItem('token') || '',
    userInfo: null as any,
    menus: [] as any[]
  }),

  actions: {
    setToken(token: string) {
      this.token = token;
      localStorage.setItem('token', token);
    },

    setUserInfo(info: any) {
      this.userInfo = info;
    },

    setMenus(menus: any[]) {
      this.menus = menus;
    },

    logout() {
      this.token = '';
      this.userInfo = null;
      this.menus = [];
      localStorage.removeItem('token');
    }
  }
});