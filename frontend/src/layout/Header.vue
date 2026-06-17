<template>
  <div class="header-box">
    <div>后台管理系统</div>

    <div>
      <el-button type="danger" size="small" @click="logout">
        退出
      </el-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'
import { useUserStore } from '@/store/user'

const router = useRouter()
const store = useUserStore()

const logout = () => {
  store.clear()
  router.push('/login')
}
</script>

<style scoped>
.header-box {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
</style>