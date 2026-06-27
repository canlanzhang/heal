<template>
  <div class="page">
    <h2>内容管理</h2>

    <button @click="goCreate">新增文章</button>

    <table border="1" cellpadding="10">
      <thead>
        <tr>
          <th>ID</th>
          <th>标题</th>
          <th>状态</th>
          <th>作者</th>
          <th>操作</th>
        </tr>
      </thead>

      <tbody>
        <tr v-for="item in list" :key="item.id">
          <td>{{ item.id }}</td>
          <td>{{ item.title }}</td>
          <td>{{ item.status }}</td>
          <td>{{ item.author_id }}</td>

          <td>
            <button @click="edit(item.id)">编辑</button>
            <button @click="remove(item.id)">删除</button>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { getArticleList, deleteArticle } from '@/api/article';

const router = useRouter();

const list = ref<any[]>([]);

const loadData = async () => {
  const res = await getArticleList();

  list.value = res?.data ?? [];
};

onMounted(() => {
  loadData();
});

const goCreate = () => {
  router.push('/article/edit');
};

const edit = (id: number) => {
  router.push(`/article/edit/${id}`);
};

const remove = async (id: number) => {
  const ok = confirm('⚠️ 确定删除该文章吗？此操作不可恢复');

  if (!ok) return;

  try {
    await deleteArticle(id);

    alert('删除成功');

    loadData();
  } catch (e) {
    alert('删除失败，请稍后重试');
  }
};
</script>

<style scoped>
.page {
  padding: 20px;
}
</style>