<template>
  <el-table :data="list" style="width: 100%">

    <el-table-column prop="id" label="ID" />
    <el-table-column prop="username" label="用户名" />
    <el-table-column prop="email" label="邮箱" />
    <el-table-column prop="role" label="角色" />

    <el-table-column label="操作">
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
defineProps({
  list: {
    type: Array,
    default: () => []
  }
})

defineEmits(['edit', 'remove'])
</script>