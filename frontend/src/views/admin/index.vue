<template>
  <div class="page">

    <el-card>
      <div style="margin-bottom: 10px;">
        <el-button type="primary" @click="openCreate">
          新增管理员
        </el-button>
      </div>

      <AdminTable
        :list="list"
        @edit="edit"
        @remove="remove"
      />
    </el-card>

  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import AdminTable from './components/AdminTable.vue'
import { getAdminList, deleteAdmin } from '@/api/admin'
import { ElMessage } from 'element-plus'

const list = ref<any[]>([])

const loadData = async () => {
  const res = await getAdminList()
  list.value = res.data.data || []
}

onMounted(loadData)

const openCreate = () => {
  console.log('open create')
}

const edit = (id: number) => {
  console.log('edit', id)
}

const remove = async (id: number) => {
  await deleteAdmin(id)
  ElMessage.success('删除成功')
  loadData()
}
</script>