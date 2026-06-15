<template>
  <div class="home">
    <h2>系统首页</h2>

    <!-- 统计卡片 -->
    <div class="card-grid">
      <div class="card">
        <p>用户数</p>
        <h3>{{ stats.users }}</h3>
      </div>

      <div class="card">
        <p>订单数</p>
        <h3>{{ stats.orders }}</h3>
      </div>

      <div class="card">
        <p>访问量</p>
        <h3>{{ stats.visits }}</h3>
      </div>
    </div>

    <!-- 最近列表 -->
    <div class="panel">
      <h3>最近登录用户</h3>

      <ul>
        <li v-for="item in recentUsers" :key="item.id">
          {{ item.name }} - {{ item.time }}
        </li>
      </ul>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';

// 模拟数据（后面可以换 API）
const stats = ref({
  users: 0,
  orders: 0,
  visits: 0
});

const recentUsers = ref([
  { id: 1, name: 'Alice', time: '2026-01-01' },
  { id: 2, name: 'Bob', time: '2026-01-02' },
  { id: 3, name: 'Charlie', time: '2026-01-03' }
]);

// 模拟接口加载
onMounted(() => {
  setTimeout(() => {
    stats.value = {
      users: 128,
      orders: 36,
      visits: 982
    };
  }, 500);
});
</script>

<style scoped>
.home {
  padding: 20px;
}

.card-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 16px;
  margin-bottom: 20px;
}

.card {
  background: #fff;
  padding: 16px;
  border-radius: 10px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.08);
}

.card h3 {
  margin: 10px 0 0;
  font-size: 22px;
  color: #409eff;
}

.panel {
  background: #fff;
  padding: 16px;
  border-radius: 10px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.08);
}
</style>