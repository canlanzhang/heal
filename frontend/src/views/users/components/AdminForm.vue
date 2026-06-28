<template>
  <el-dialog
    v-model="visible"
    :title="form.id ? '编辑管理员' : '新增管理员'"
    width="500px"
  >

    <el-form :model="form" label-width="80px">

      <el-form-item label="用户名">
        <el-input v-model="form.username" />
      </el-form-item>

      <el-form-item label="邮箱">
        <el-input v-model="form.email" />
      </el-form-item>

      <el-form-item label="角色">
        <el-input v-model="form.role" />
      </el-form-item>

      <!-- 🔥 新增密码 -->
      <el-form-item label="密码" v-if="!form.id">
        <el-input v-model="form.password" type="password" />
      </el-form-item>

      <!-- 编辑密码（可选） -->
      <el-form-item label="密码" v-else>
        <el-input
          v-model="form.password"
          type="password"
          placeholder="不修改可不填"
        />
      </el-form-item>

    </el-form>

    <template #footer>
      <el-button @click="visible = false">取消</el-button>
      <el-button type="primary" @click="submit">
        确定
      </el-button>
    </template>

  </el-dialog>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue'
import {
  createAdmin,
  updateAdmin,
  getAdminById
} from '@/api/admin'

const emit = defineEmits(['success'])

const visible = ref(false)

const form = reactive<any>({
  id: null,
  username: '',
  email: '',
  role: '',
  password: ''
})

// 新增
const openCreate = () => {
  Object.assign(form, {
    id: null,
    username: '',
    email: '',
    role: '',
    password: ''
  })
  visible.value = true
}

// 编辑
const openEdit = async (id: number) => {
  const res = await getAdminById(id)
  console.dir(res.data);
  Object.assign(form, res.data.user)
  form.password = '' // 不回显密码
  visible.value = true
}

// 提交
const submit = async () => {
  if (form.id) {
    const payload: any = {
      username: form.username,
      email: form.email,
      role: form.role
    }

    // 有密码才更新
    if (form.password) {
      payload.password = form.password
    }

    await updateAdmin(form.id, payload)
  } else {
    await createAdmin(form)
  }

  visible.value = false
  emit('success')
}

defineExpose({
  openCreate,
  openEdit
})
</script>