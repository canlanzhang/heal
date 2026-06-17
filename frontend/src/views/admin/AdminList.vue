<template>
  <div class="page">

    <div class="header">
      <h2>管理员管理</h2>
      <button @click="openAdd">+ 新增管理员</button>
    </div>

    <!-- 表格 -->
    <table>
      <thead>
        <tr>
          <th>ID</th>
          <th>用户名</th>
          <th>角色</th>
          <th>操作</th>
        </tr>
      </thead>

      <tbody>
        <tr v-for="item in list" :key="item.id">
          <td>{{ item.id }}</td>
          <td>{{ item.username }}</td>
          <td>{{ item.role }}</td>
          <td>
            <button @click="openEdit(item)">编辑</button>
            <button @click="remove(item.id)">删除</button>
          </td>
        </tr>
      </tbody>
    </table>

    <!-- 弹窗 -->
    <div v-if="showDialog" class="modal">
      <div class="box">

        <h3>{{ form.id ? '编辑管理员' : '新增管理员' }}</h3>

        <input v-model="form.username" placeholder="用户名" />
        <input v-model="form.password" placeholder="密码" />

        <select v-model="form.role">
          <option value="admin">admin</option>
          <option value="user">user</option>
        </select>

        <div class="actions">
          <button @click="save">确定</button>
          <button @click="close">取消</button>
        </div>

      </div>
    </div>

  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';

import {
  getAdminList,
  createAdmin,
  updateAdmin,
  deleteAdmin
} from '@/api/admin';

const list = ref<any[]>([]);
const showDialog = ref(false);
const loading = ref(false);

const form = reactive<any>({
  id: null,
  username: '',
  password: '',
  role: 'admin'
});

/**
 * 获取列表（核心）
 */
const fetchList = async () => {
  loading.value = true;

  try {
    const res = await getAdminList({
      page: 1,
      size: 10
    });

    // 假设后端：{ data: { list: [] } }
    list.value = res?.data?.list || res?.data || [];

  } finally {
    loading.value = false;
  }
};

onMounted(() => {
  fetchList();
});

/**
 * 新增
 */
const openAdd = () => {
  form.id = null;
  form.username = '';
  form.password = '';
  form.role = 'admin';
  showDialog.value = true;
};

/**
 * 编辑
 */
const openEdit = (item: any) => {
  Object.assign(form, item);
  showDialog.value = true;
};

/**
 * 保存（新增/修改）
 */
const save = async () => {
  if (form.id) {
    await updateAdmin(form.id, form);
  } else {
    await createAdmin(form);
  }

  showDialog.value = false;
  fetchList();
};

/**
 * 删除
 */
const remove = async (id: number) => {
  if (!confirm('确定删除吗？')) return;

  await deleteAdmin(id);
  fetchList();
};

/**
 * 关闭弹窗
 */
const close = () => {
  showDialog.value = false;
};
</script>

<style scoped>
.page {
  padding: 20px;
}

.header {
  display: flex;
  justify-content: space-between;
  margin-bottom: 10px;
}

table {
  width: 100%;
  border-collapse: collapse;
  background: #fff;
}

th, td {
  border: 1px solid #eee;
  padding: 10px;
}

button {
  margin-right: 5px;
}

.modal {
  position: fixed;
  inset: 0;
  background: rgba(0,0,0,0.3);
  display: flex;
  align-items: center;
  justify-content: center;
}

.box {
  background: #fff;
  padding: 20px;
  width: 320px;
}

.actions {
  margin-top: 10px;
  display: flex;
  justify-content: space-between;
}
</style>