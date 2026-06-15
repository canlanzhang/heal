<template>
  <div class="sidebar">
    <h3>后台系统</h3>

    <router-link
      v-for="item in menus"
      :key="item.path"
      :to="item.path"
    >
      {{ item.title }}
    </router-link>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useUserStore } from '@/store/user';

const userStore = useUserStore();
const menus = computed(() => userStore.menus);
</script>