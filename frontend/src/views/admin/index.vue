<template>
  <div class="page">

    <el-card>

      <el-button type="primary" @click="openCreate">
        新增管理员
      </el-button>

      <AdminTable
        :list="list"
        @edit="openEdit"
        @remove="handleRemove"
      />

      <AdminForm
        ref="formRef"
        @success="loadData"
      />

    </el-card>

  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import AdminTable from './components/AdminTable.vue'
import AdminForm from './components/AdminForm.vue'

import {
  getAdminList,
  deleteAdmin
} from '@/api/admin'

import { ElMessage, ElMessageBox } from 'element-plus'

const list = ref<any[]>([])
const formRef = ref()

const loadData = async () => {
  const res = await getAdminList()
  list.value = res.data || []
}

onMounted(loadData)

// 新增
const openCreate = () => {
  formRef.value.openCreate()
}

// 编辑
const openEdit = (id: number) => {
  formRef.value.openEdit(id)
}

// 删除
const handleRemove = async (id: number) => {
  await ElMessageBox.confirm('确认删除该管理员？', '提示', {
    type: 'warning'
  })

  await deleteAdmin(id)
  ElMessage.success('删除成功')
  loadData()
}
</script>