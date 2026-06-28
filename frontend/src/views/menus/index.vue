<template>
  <div class="page">
    <el-card>

      <el-button type="primary" @click="openCreate">
        新增菜单
      </el-button>

      <MenuTable
        :list="list"
        @edit="openEdit"
        @remove="handleRemove"
      />

      <MenuForm
        ref="formRef"
        @success="loadData"
      />

    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";

import MenuTable from "./components/MenuTable.vue";
import MenuForm from "./components/MenuForm.vue";

import {
  listMenus,
  deleteMenu
} from "@/api/menu";

import { ElMessage, ElMessageBox } from "element-plus";

const list = ref<any[]>([]);
const formRef = ref();

// 加载数据
const loadData = async () => {
  const res = await listMenus();
  list.value = res.data || [];
};

onMounted(loadData);

// 新增
const openCreate = () => {
  formRef.value.openCreate();
};

// 编辑
const openEdit = (id: number) => {
  formRef.value.openEdit(id);
};

// 删除
const handleRemove = async (id: number) => {
  await ElMessageBox.confirm("确认删除该菜单？", "提示", {
    type: "warning",
  });

  await deleteMenu(id);

  ElMessage.success("删除成功");
  loadData();
};
</script>