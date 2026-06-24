<template>
  <div class="login-page">
    <el-card class="login-box">
      <h2>后台登录</h2>

      <el-form
        :model="form"
        label-width="80px"
        @submit.prevent="handleLogin"
      >
        <el-form-item label="用户名">
          <el-input v-model="form.username" />
        </el-form-item>

        <el-form-item label="密码">
          <el-input v-model="form.password" type="password" />
        </el-form-item>

        <el-button
          type="primary"
          :loading="loading"
          native-type="submit"
        >
          登录
        </el-button>
      </el-form>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { useUserStore } from '@/store/user'
import { loginApi } from '@/api/auth'

const router = useRouter()
const store = useUserStore()

const loading = ref(false)

const form = reactive({
  username: '',
  password: ''
})

const handleLogin = async () => {
  if (loading.value) return

  try {
    loading.value = true

    const res = await loginApi(form)

    const token = res?.data?.token

    if (!token) {
      ElMessage.error('token 获取失败')
      return
    }

    // 1️⃣ 存 token
    store.setToken(token)


    // 2️⃣ ⭐关键：用 token 拉 menus
    await store.fetchProfile()

    console.log('menus:', store.menus)

    ElMessage.success('登录成功')

    // ❗只做跳转，不做任何路由注册
    router.push('/')
  } catch (e) {
    console.error(e)
    ElMessage.error('登录失败')
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.login-page {
  height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
  background: #f5f5f5;
}

.login-box {
  width: 360px;
}
</style>