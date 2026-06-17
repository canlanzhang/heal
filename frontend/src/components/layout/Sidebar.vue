<template>
  <div class="sidebar">

    <div class="logo">
      后台系统
    </div>

    <nav class="menu">
      <router-link
        v-for="item in menus"
        :key="item.path"
        :to="item.path"
        class="menu-item"
      >
        {{ item.title }}
      </router-link>
    </nav>

  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useUserStore } from '@/store/user';

const userStore = useUserStore();

const menus = computed(() => userStore.menus || []);
</script>

<style scoped>
.sidebar {
  width: 220px;
  background: #001529;
  color: #fff;
  height: 100vh;
  display: flex;
  flex-direction: column;
}

.logo {
  padding: 16px;
  font-size: 16px;
  font-weight: bold;
  border-bottom: 1px solid #222;
}

/* ⭐关键：必须竖排 */
.menu {
  display: flex;
  flex-direction: column;
  padding: 10px;
  gap: 8px;
}

.menu-item {
  color: #fff;
  text-decoration: none;
  padding: 10px 12px;
  border-radius: 6px;
  display: block; /* ⭐关键 */
}

.menu-item:hover {
  background: #1890ff;
}
</style>