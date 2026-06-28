<template>
  <div class="page">

    <el-card>

      <div style="margin-bottom: 10px;">
        <el-button type="primary" @click="openCreate">
          新增文章
        </el-button>
      </div>

      <ArticleTable
        :list="list"
        @edit="openEdit"
        @remove="remove"
      />

      <ArticleForm
        ref="formRef"
        @submit="handleSubmit"
      />

    </el-card>

  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'

import ArticleTable from './components/ArticleTable.vue'
import ArticleForm from './components/ArticleForm.vue'

import {
  getArticleList,
  getArticleById,
  createArticle,
  updateArticle,
  deleteArticle
} from '@/api/article'

const list = ref<any[]>([])
const formRef = ref()

// 加载列表
const loadData = async () => {
  const res = await getArticleList()
  list.value = res.data || []
}

onMounted(loadData)

// 新增
const openCreate = () => {
  formRef.value.openCreate()
}

// 编辑
const openEdit = async (id: number) => {
  const res = await getArticleById(id)
  formRef.value.openEdit(res.data)
}

// 提交
const handleSubmit = async (form: any) => {
  if (form.id) {
    await updateArticle(form.id, form)
    ElMessage.success('更新成功')
  } else {
    await createArticle(form)
    ElMessage.success('创建成功')
  }

  formRef.value.close()
  loadData()
}

// 删除
const remove = async (id: number) => {
  await ElMessageBox.confirm('确定删除该文章吗？', '提示', {
    type: 'warning'
  })

  await deleteArticle(id)
  ElMessage.success('删除成功')
  loadData()
}
</script>