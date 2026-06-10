<template>
  <div class="login-container">
    <h2 class="title">用户登录</h2>
    <form @submit.prevent="handleLogin" class="login-form">
      <!-- 用户名输入框 -->
      <div class="input-group">
        <label for="username">用户名</label>
        <input
          type="text"
          id="username"
          v-model="username"
          placeholder="请输入用户名"
          required
        />
      </div>

      <!-- 密码输入框 -->
      <div class="input-group">
        <label for="password">密码</label>
        <input
          type="password"
          id="password"
          v-model="password"
          placeholder="请输入密码"
          required
        />
      </div>

      <!-- 错误提示 (新增) -->
      <div v-if="errorMessage" class="error-msg">{{ errorMessage }}</div>

      <!-- 登录按钮 -->
      <button type="submit" :disabled="isSubmitting">
        {{ isSubmitting ? '登录中...' : '登录' }}
      </button>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import axios from 'axios'; // 1. 引入 axios
import { useRouter } from 'vue-router'; // 引入路由用于跳转

const router = useRouter();

// 表单数据
const username = ref('');
const password = ref('');
// 状态控制
const isSubmitting = ref(false);
const errorMessage = ref(''); // 用于显示后端返回的错误信息

// --- 配置区 ---
// 请在这里填入你真实的后端接口地址
// 例如: http://localhost:8080/api/login 或 /api/user/login
const API_LOGIN_URL = 'https://localhost:8443/api/login';
// ----------------

// 登录处理逻辑
const handleLogin = async () => {
  // 重置错误信息
  errorMessage.value = '';

  if (!username.value || !password.value) {
    errorMessage.value = '请输入用户名和密码！';
    return;
  }

  isSubmitting.value = true;

  try {
    // 2. 发送真实的 POST 请求
    const response = await axios.post(API_LOGIN_URL, {
      username: username.value,
      password: password.value
    });

    // 3. 处理成功响应
    // 假设后端返回的数据结构是 { code: 200, data: { token: 'xxx' } }
    // 具体结构需要根据你的后端文档调整
    if (response.data && response.data.token) {
      // 存储 Token
      localStorage.setItem('token', response.data.token);

      // 跳转到首页
      // 注意：这里使用 router.push 而不是 window.location.href，体验更好（无刷新）
      router.push('/home');
    } else {
      // 如果请求成功但没拿到 token，可能是业务逻辑错误
      errorMessage.value = response.data.message || '登录失败，未获取到凭证';
    }

  } catch (error: any) {
    // 4. 处理错误响应
    console.error('Login Error:', error);

    if (error.response) {
      // 服务器返回了状态码（如 401, 500）
      errorMessage.value = error.response.data.message || `服务器错误: ${error.response.status}`;
    } else if (error.request) {
      // 请求发出去了但没有收到响应（如跨域、断网、后端没启动）
      errorMessage.value = '无法连接到服务器，请检查网络或后端服务是否启动';
    } else {
      errorMessage.value = '请求发生未知错误';
    }
  } finally {
    isSubmitting.value = false;
  }
};
</script>

<style scoped>
/* 保持原有样式不变 */
.login-container {
  max-width: 400px;
  margin: 100px auto;
  padding: 20px;
  border: 1px solid #ddd;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.title {
  text-align: center;
  margin-bottom: 20px;
  color: #333;
}

.input-group {
  margin-bottom: 15px;
}

.input-group label {
  display: block;
  margin-bottom: 5px;
  font-weight: bold;
  color: #555;
}

.input-group input {
  width: 100%;
  padding: 8px;
  border: 1px solid #ccc;
  border-radius: 4px;
  box-sizing: border-box;
}

button {
  width: 100%;
  padding: 10px;
  background-color: #4caf50;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.3s;
}

button:hover:not(:disabled) {
  background-color: #45a049;
}

button:disabled {
  background-color: #a5d6a7;
  cursor: not-allowed;
}

/* 新增错误提示样式 */
.error-msg {
  color: #ff4d4f;
  margin-bottom: 15px;
  font-size: 14px;
  text-align: center;
}
</style>