<template>
  <div class="login-page">
    <el-card class="login-box">
      <h2>后台登录</h2>

      <el-form 
        :model="form" 
        label-width="80px"
        @submit.prevent="handleLogin">
        <el-form-item label="用户名">
          <el-input v-model="form.username" />
        </el-form-item>

        <el-form-item label="密码">
          <el-input v-model="form.password" type="password" />
        </el-form-item>

        <el-button 
          type="primary" 
          :loading="loading"
          native-type="submit">
          登录
        </el-button>
      </el-form>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue';
import { useRouter } from 'vue-router';
import { ElMessage } from 'element-plus';
import { useUserStore } from '@/store/user';
import { bootstrap } from '@/bootstrap'
import { loginApi, getProfileApi } from '@/api/auth';
import { generateRoutes } from '@/utils/dynamicRoute';

const router = useRouter();
const userStore = useUserStore();

const loading = ref(false)

// 表单
const form = reactive({
  username: '',
  password: ''
});

// 状态
//const loading = ref(false);
//const errorMsg = ref('');

// 登录逻辑
const handleLogin = async () => {
  //errorMsg.value = '';

  if (!form.username || !form.password) {
    ElMessage.warning('请输入用户名和密码')
    return
  }

  loading.value = true

  try {

    // 1️⃣ 登录
    const loginRes = await loginApi({
      username: form.username,
      password: form.password
    });

    // 1️⃣ 调登录接口
    const token = loginRes?.data?.token;
//console.dir(loginRes, { depth: null, colors: true });

    // 2️⃣ 保存 token
    userStore.token = token
    localStorage.setItem('token', token)

    ElMessage.success('登录成功')

    // 3️⃣ 拉用户信息 + menus（后端返回）
    await userStore.fetchProfile()

    // 4️⃣ 初始化动态路由（关键！！！）
    await bootstrap()

    // 5️⃣ 跳转首页（必须在 bootstrap 后）
    router.push('/dashboard')

  } catch (err: any) {
    console.log("err: "+err);
    ElMessage.error('登录失败，请检查账号密码')
  } finally {
    loading.value = false;
  }
};
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