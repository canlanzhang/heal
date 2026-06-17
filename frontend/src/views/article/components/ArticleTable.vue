<template>
  <el-table :data="list" border style="width: 100%">

    <el-table-column prop="id" label="ID" width="80" />

    <el-table-column prop="title" label="标题" />

    <el-table-column prop="status" label="状态" width="120" />

    <el-table-column prop="created_at" label="创建时间" width="180" />

    <el-table-column label="操作" width="200">
      <template #default="{ row }">
        <el-button size="small" @click="$emit('edit', row.id)">
          编辑
        </el-button>

        <el-button
          size="small"
          type="danger"
          @click="$emit('remove', row.id)"
        >
          删除
        </el-button>
      </template>
    </el-table-column>

  </el-table>
</template>

<script setup lang="ts">
defineProps<{
  list: any[]
}>()
</script>