<template>
  <el-container class="layout">

    <!-- 左侧 -->
    <el-aside width="220px" class="aside">
      <Sidebar />
    </el-aside>

    <!-- 右侧 -->
    <el-container>

      <!-- 顶部 -->
      <el-header class="header">
        <Header />
      </el-header>

      <!-- 内容 -->
      <el-main class="main">
        <router-view />
      </el-main>

    </el-container>

  </el-container>
</template>

<script setup lang="ts">
import Sidebar from './Sidebar.vue'
import Header from './Header.vue'
</script>

<style scoped>
.layout {
  height: 100vh;
}

.aside {
  background: #001529;
  color: #fff;
}

.header {
  background: #fff;
  border-bottom: 1px solid #eee;
}

.main {
  background: #f5f6f8;
}
</style>