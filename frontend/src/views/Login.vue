<template>
  <div class="login-container">
    <h2>后台系统登录</h2>

    <form @submit.prevent="handleLogin">
      <!-- 用户名 -->
      <div class="input-group">
        <label>用户名</label>
        <input v-model="form.username" type="text" placeholder="请输入用户名" />
      </div>

      <!-- 密码 -->
      <div class="input-group">
        <label>密码</label>
        <input v-model="form.password" type="password" placeholder="请输入密码" />
      </div>

      <!-- 错误提示 -->
      <div v-if="errorMsg" class="error">
        {{ errorMsg }}
      </div>

      <!-- 登录按钮 -->
      <button type="submit" :disabled="loading">
        {{ loading ? '登录中...' : '登录' }}
      </button>
    </form>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue';
import { useRouter } from 'vue-router';
import { useUserStore } from '@/store/user';
import { loginApi, getUserInfoApi } from '@/api/auth';
import { generateRoutes } from '@/utils/dynamicRoute';

const router = useRouter();
const userStore = useUserStore();

// 表单
const form = reactive({
  username: '',
  password: ''
});

// 状态
const loading = ref(false);
const errorMsg = ref('');

// 登录逻辑
const handleLogin = async () => {
  errorMsg.value = '';

  if (!form.username || !form.password) {
    errorMsg.value = '请输入用户名和密码';
    return;
  }

  try {
    loading.value = true;

    // 1️⃣ 登录
    const loginRes = await loginApi({
      username: form.username,
      password: form.password
    });

    const token = loginRes?.data?.token;

    if (!token) {
      errorMsg.value = loginRes?.message || '登录失败';
      return;
    }

    // 2️⃣ 存 token
    userStore.setToken(token);

    // 3️⃣ 获取用户信息 + 菜单
    const infoRes = await getUserInfoApi();

    const user = infoRes?.data?.user;
    const menus = infoRes?.data?.menus || [];

    userStore.setUserInfo(user);
    userStore.setMenus(menus);

    // 4️⃣ 动态注册路由（核心）
    generateRoutes(menus);

    // 5️⃣ 跳转首页
    router.push('/home');

  } catch (err: any) {
    errorMsg.value =
      err?.response?.data?.message ||
      '网络异常，请稍后重试';
  } finally {
    loading.value = false;
  }
};
</script>

<style scoped>
.login-container {
  width: 360px;
  margin: 120px auto;
  padding: 24px;
  border-radius: 10px;
  background: #fff;
  box-shadow: 0 4px 20px rgba(0,0,0,0.1);
}

h2 {
  text-align: center;
  margin-bottom: 20px;
}

.input-group {
  margin-bottom: 12px;
}

.input-group input {
  width: 100%;
  padding: 8px;
  border: 1px solid #ddd;
  border-radius: 6px;
}

button {
  width: 100%;
  padding: 10px;
  background: #409eff;
  border: none;
  color: #fff;
  border-radius: 6px;
  cursor: pointer;
}

button:disabled {
  background: #a0cfff;
}

.error {
  color: red;
  margin-bottom: 10px;
  font-size: 13px;
}
</style>