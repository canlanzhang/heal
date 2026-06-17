<template>
  <div class="page">

    <el-card>

      <div style="margin-bottom: 10px;">
        <el-button type="primary" @click="create">
          新增文章
        </el-button>
      </div>

      <ArticleTable
        :list="list"
        @edit="edit"
        @remove="remove"
      />

    </el-card>

  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import ArticleTable from './components/ArticleTable.vue'
import { getArticleList, deleteArticle } from '@/api/article'
import { ElMessage } from 'element-plus'

const list = ref<any[]>([])

const loadData = async () => {
  const res = await getArticleList()
  list.value = res.data || []
}

onMounted(loadData)

const create = () => {
  console.log('create article')
}

const edit = (id: number) => {
  console.log('edit', id)
}

const remove = async (id: number) => {
  await deleteArticle(id)
  ElMessage.success('删除成功')
  loadData()
}
</script>