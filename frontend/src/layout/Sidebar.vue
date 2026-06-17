<template>
  <el-menu
    :default-active="active"
    class="menu"
    background-color="#001529"
    text-color="#fff"
    router
  >
    <el-menu-item
      v-for="item in menus"
      :key="item.path"
      :index="item.path"
    >
      {{ item.title }}
    </el-menu-item>
  </el-menu>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useUserStore } from '@/store/user'
import { useRoute } from 'vue-router'

const store = useUserStore()
const route = useRoute()

const menus = computed(() => store.menus || [])
const active = computed(() => route.path)
</script>

<style scoped>
.menu {
  height: 100%;
}
</style>