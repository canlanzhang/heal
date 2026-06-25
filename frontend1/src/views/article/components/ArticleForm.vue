<template>
  <el-dialog v-model="visible" title="文章表单" width="600px">

    <el-form :model="form" label-width="80px">

      <el-form-item label="标题">
        <el-input v-model="form.title" />
      </el-form-item>

      <el-form-item label="内容">
        <el-input
          v-model="form.content"
          type="textarea"
          rows="6"
        />
      </el-form-item>

      <el-form-item label="状态">
        <el-select v-model="form.status" style="width: 100%">
          <el-option label="草稿" value="draft" />
          <el-option label="发布" value="published" />
        </el-select>
      </el-form-item>

    </el-form>

    <template #footer>
      <el-button @click="visible = false">取消</el-button>
      <el-button type="primary" @click="submit">确定</el-button>
    </template>

  </el-dialog>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue'

const emit = defineEmits(['submit'])

const visible = ref(false)

const form = reactive<any>({
  id: undefined,
  title: '',
  content: '',
  status: 'draft'
})

const openCreate = () => {
  Object.assign(form, {
    id: undefined,
    title: '',
    content: '',
    status: 'draft'
  })
  visible.value = true
}

const openEdit = (data: any) => {
  Object.assign(form, data)
  visible.value = true
}

const submit = () => {
  emit('submit', { ...form })
}

const close = () => {
  visible.value = false
}

defineExpose({
  openCreate,
  openEdit,
  close
})
</script>