<template>
  <div class="page">
    <h2>{{ isEdit ? '编辑文章' : '新增文章' }}</h2>

    <div>
      <input v-model="form.title" placeholder="标题" />
    </div>

    <div>
      <textarea v-model="form.content" placeholder="内容"></textarea>
    </div>

    <div>
      <select v-model="form.status">
        <option value="draft">草稿</option>
        <option value="published">发布</option>
      </select>
    </div>

    <button @click="submit">提交</button>
  </div>
</template>

<script setup lang="ts">
import { reactive, onMounted, computed } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import {
  createArticle,
  updateArticle,
  getArticleById,
} from '@/api/article';

const route = useRoute();
const router = useRouter();

const id = Number(route.params.id);

const isEdit = computed(() => !!id);

const form = reactive({
  title: '',
  content: '',
  status: 'draft',
});

onMounted(async () => {
  if (isEdit.value) {
    const res = await getArticleById(id);
    Object.assign(form, res.data);
  }
});

const submit = async () => {
  if (isEdit.value) {
    await updateArticle(id, form);
  } else {
    await createArticle(form);
  }

  router.push('/article');
};
</script>

<style scoped>
.page {
  padding: 20px;
}

input, textarea, select {
  width: 300px;
  margin: 10px 0;
}
textarea {
  height: 150px;
}
</style>