<script setup lang="ts">
import HelloWorld from './components/HelloWorld.vue'

import { ref, onMounted } from 'vue'

const userInfo = ref<any>(null)
  

const message = ref('等待后端响应...')

onMounted(async () => {
  try {
    // 假设你在 Axum 里有一个 GET /api/hello 的接口
    const res = await fetch('/api/user/1')
    const data = await res.json()
    userInfo.value = data.data
  } catch (e) {
    console.error("连接失败", e)
  }
})

</script>

<template>asd
  <div v-if="userInfo" class="card">
      <h2>后端返回的用户：</h2>
      <p>ID: {{ userInfo.id }}</p>
      <p>用户名: {{ userInfo.username }}</p>
      <p>创建时间: {{ userInfo.created_at }}</p>
      <p>修改时间: {{ userInfo.updated_at }}</p>
    </div>
  <HelloWorld />

</template>
