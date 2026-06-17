<template>
  <div class="home">
    <el-card>
      <h2>欢迎使用后台系统</h2>
      <p>当前登录用户：{{ user?.username }}</p>
      <p>角色：{{ user?.role }}</p>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useUserStore } from '@/store/user'

const store = useUserStore()
const user = computed(() => store.user)
</script>

<style scoped>
.home {
  padding: 20px;
}
</style>