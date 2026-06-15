<template>
  <div class="header">
    <div>系统管理后台</div>

    <div class="right">
      <span>admin</span>
      <button @click="logout">退出</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';

const router = useRouter();

const logout = () => {
  localStorage.removeItem('token');
  router.push('/login');
};
</script>

<style scoped>
.header {
  height: 50px;
  background: #fff;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 16px;
  border-bottom: 1px solid #eee;
}

.right {
  display: flex;
  gap: 10px;
  align-items: center;
}
</style>