<template>
  <el-menu
    class="menu"
    background-color="#001529"
    text-color="#fff"
    :default-active="active"
    router
  >
    <el-menu-item
      v-for="item in menus"
      :key="item.path"
      :index="item.path"
      @click="go(item)"
    >
      {{ item.title || item.name || item.path }}
    </el-menu-item>
  </el-menu>
</template>

<script setup lang="ts">
import { computed, watchEffect } from 'vue'
import { useUserStore } from '@/store/user'
import { useRoute, useRouter } from 'vue-router'

const store = useUserStore()
const route = useRoute()
const router = useRouter()

// ✅ 必须确保响应式
const menus = computed(() => store.menus)

const active = computed(() => route.path)

// 🔥 调试：确认菜单是否进来了
watchEffect(() => {
  console.log('🧠 menus updated:', menus.value)
})

const go = (item: any) => {
  console.log('🟡 click menu:', item)

  router.push(item.path)
}
</script>

<style scoped>
.menu {
  height: 100%;
}
</style>