<template>
  <div class="header">

    <!-- 左侧 -->
    <div class="left">
      <el-breadcrumb separator="/">
        <el-breadcrumb-item>首页</el-breadcrumb-item>
        <el-breadcrumb-item>{{ route.meta.title || '控制台' }}</el-breadcrumb-item>
      </el-breadcrumb>
    </div>

    <!-- 右侧 -->
    <div class="right">

      <!-- 刷新 -->
      <el-tooltip content="刷新页面">
        <el-button circle @click="reload">
          <el-icon><Refresh /></el-icon>
        </el-button>
      </el-tooltip>

      <!-- 全屏 -->
      <el-tooltip content="全屏">
        <el-button circle @click="toggleFullScreen">
          <el-icon><FullScreen /></el-icon>
        </el-button>
      </el-tooltip>

      <!-- 消息 -->
      <el-badge :value="3" class="item">
        <el-button circle>
          <el-icon><Bell /></el-icon>
        </el-button>
      </el-badge>

      <!-- 用户 -->
      <el-dropdown @command="handleCommand">

        <span class="user-box">
          <el-avatar size="small">
            {{ user?.username?.charAt(0) || 'U' }}
          </el-avatar>

          <span class="username">
            {{ user?.username || '未登录' }}
          </span>

          <el-icon><ArrowDown /></el-icon>
        </span>

        <template #dropdown>
          <el-dropdown-menu>

            <el-dropdown-item command="profile">
              个人中心
            </el-dropdown-item>

            <el-dropdown-item command="password">
              修改密码
            </el-dropdown-item>

            <el-dropdown-item command="logout" divided>
              退出登录
            </el-dropdown-item>

          </el-dropdown-menu>
        </template>

      </el-dropdown>

    </div>

  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useUserStore } from '@/store/user'
import {
  ElMessageBox,
  ElMessage
} from 'element-plus'

import {
  Refresh,
  FullScreen,
  Bell,
  ArrowDown
} from '@element-plus/icons-vue'

const route = useRoute()
const router = useRouter()
const store = useUserStore()

const user = computed(() => store.user)

/**
 * 刷新页面（只刷新当前路由组件）
 */
const reload = () => {
  location.reload()
}

/**
 * 全屏切换
 */
const toggleFullScreen = () => {
  const doc: any = document

  if (!doc.fullscreenElement) {
    doc.documentElement.requestFullscreen()
  } else {
    doc.exitFullscreen()
  }
}

/**
 * 下拉菜单
 */
const handleCommand = async (command: string) => {
  switch (command) {

    case 'logout':
      await ElMessageBox.confirm('确认退出登录？', '提示', {
        type: 'warning'
      })

      store.logout()
      router.push('/login')

      break

    case 'profile':
      ElMessage.info('个人中心开发中')
      break

    case 'password':
      ElMessage.info('修改密码开发中')
      break
  }
}
</script>

<style scoped>
.header {
  height: 60px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 20px;
  background: #fff;
  border-bottom: 1px solid #eee;
}

.left {
  display: flex;
  align-items: center;
}

.right {
  display: flex;
  align-items: center;
  gap: 12px;
}

.user-box {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  padding: 0 10px;
}

.username {
  font-size: 14px;
}
</style>