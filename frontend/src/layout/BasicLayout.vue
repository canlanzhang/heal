<template>
  <div class="layout">

    <!-- 左侧菜单 -->
    <Sidebar />

    <div class="main">

      <!-- 顶部栏 -->
      <Header />

      <!-- 内容区 -->
      <div class="content">
        <router-view />
      </div>

    </div>
  </div>
</template>

<script setup lang="ts">
import Sidebar from '@/components/layout/Sidebar.vue';
import Header from '@/components/layout/Header.vue';
</script>

<style scoped>
.layout {
  display: flex;
  height: 100vh;
  background: #f5f6f8;
}

.main {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.content {
  flex: 1;
  padding: 16px;
  overflow: auto;
}
</style>