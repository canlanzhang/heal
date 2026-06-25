<template>
  <el-table :data="list" border style="width: 100%">

    <el-table-column prop="id" label="ID" width="80" />

    <el-table-column prop="name" label="Name" />

    <el-table-column prop="path" label="路径" />

    <el-table-column prop="title" label="标题" />

    <el-table-column prop="icon" label="图标" />

    <el-table-column prop="role" label="角色" />

    <el-table-column prop="sort" label="排序" width="80" />

    <el-table-column label="操作" width="200">
      <template #default="{ row }">

        <el-button type="primary" size="small" @click="$emit('edit', row.id)">
          编辑
        </el-button>

        <el-button type="danger" size="small" @click="$emit('remove', row.id)">
          删除
        </el-button>

      </template>
    </el-table-column>

  </el-table>
</template>

<script setup lang="ts">
defineProps({
  list: {
    type: Array,
    default: () => [],
  },
});

defineEmits(["edit", "remove"]);
</script>