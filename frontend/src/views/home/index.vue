<template>
  <div class="home">

    <!-- 顶部统计 -->
    <el-row :gutter="20">

      <el-col :span="6">
        <el-card>
          <div class="card-title">管理员</div>
          <div class="card-number">{{ stats.adminCount }}</div>
        </el-card>
      </el-col>

      <el-col :span="6">
        <el-card>
          <div class="card-title">文章</div>
          <div class="card-number">{{ stats.articleCount }}</div>
        </el-card>
      </el-col>

      <el-col :span="6">
        <el-card>
          <div class="card-title">用户</div>
          <div class="card-number">{{ stats.userCount }}</div>
        </el-card>
      </el-col>

      <el-col :span="6">
        <el-card>
          <div class="card-title">访问量</div>
          <div class="card-number">{{ stats.visitCount }}</div>
        </el-card>
      </el-col>

    </el-row>

    <!-- 快捷入口 -->
    <el-card style="margin-top: 20px;">
      <div style="display: flex; gap: 10px;">
        <el-button type="primary">新增文章</el-button>
        <el-button>管理员管理</el-button>
        <el-button>内容管理</el-button>
      </div>
    </el-card>

    <!-- 最近文章 -->
    <el-card style="margin-top: 20px;">
      <template #header>
        <span>最新文章</span>
      </template>

      <el-table :data="latestArticles">
        <el-table-column prop="title" label="标题" />
        <el-table-column prop="status" label="状态" />
        <el-table-column prop="created_at" label="时间" />
      </el-table>

    </el-card>

  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

const stats = ref({
  adminCount: 0,
  articleCount: 0,
  userCount: 0,
  visitCount: 0
})

const latestArticles = ref<any[]>([])

// 模拟数据（后面接接口）
const loadData = async () => {

  // 👉 后面这里换成真实接口
  stats.value = {
    adminCount: 3,
    articleCount: 12,
    userCount: 58,
    visitCount: 1024
  }

  latestArticles.value = [
    { title: '系统上线', status: 'published', created_at: '2025-01-01' },
    { title: '后台优化', status: 'draft', created_at: '2025-01-02' }
  ]
}

onMounted(loadData)
</script>

<style scoped>
.card-title {
  font-size: 14px;
  color: #999;
}

.card-number {
  font-size: 28px;
  font-weight: bold;
  margin-top: 10px;
}
</style>