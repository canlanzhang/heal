<template>
  <el-dialog v-model="visible" title="菜单表单" width="500px">

    <el-form :model="form" label-width="80px">

      <el-form-item label="Name">
        <el-input v-model="form.name" />
      </el-form-item>

      <el-form-item label="Path">
        <el-input v-model="form.path" />
      </el-form-item>

      <el-form-item label="Title">
        <el-input v-model="form.title" />
      </el-form-item>

      <el-form-item label="Icon">
        <el-input v-model="form.icon" />
      </el-form-item>

      <el-form-item label="Role">
        <el-input v-model="form.role" />
      </el-form-item>

      <el-form-item label="Sort">
        <el-input v-model.number="form.sort" />
      </el-form-item>

    </el-form>

    <template #footer>
      <el-button @click="visible = false">取消</el-button>
      <el-button type="primary" @click="submit">保存</el-button>
    </template>

  </el-dialog>
</template>

<script setup lang="ts">
import { ref } from "vue";
import {
  createMenu,
  updateMenu,
  getMenuById
} from "@/api/menu";

const emit = defineEmits(["success"]);

const visible = ref(false);

const form = ref<any>({
  id: null,
  name: "",
  path: "",
  title: "",
  icon: "",
  role: "admin",
  sort: 1,
});

// 新增
const openCreate = () => {
  form.value = {
    id: null,
    name: "",
    path: "",
    title: "",
    icon: "",
    role: "admin",
    sort: 1,
  };

  visible.value = true;
};

// 编辑
const openEdit = async (id: number) => {
  const res = await getMenuById(id);

  form.value = res.data;

  visible.value = true;
};

// 提交
const submit = async () => {
  if (form.value.id) {
    await updateMenu(form.value.id, form.value);
  } else {
    await createMenu(form.value);
  }

  visible.value = false;
  emit("success");
};

// 暴露方法给父组件
defineExpose({
  openCreate,
  openEdit,
});
</script>