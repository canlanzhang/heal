import request from '@/utils/request'

// 获取列表
export const getAdminList = () => {
  return request.get('/v1/users')
}

// 详情
export const getAdminById = (id: number) => {
  return request.get(`/v1/users/${id}`)
}

// 新增
export const createAdmin = (data: any) => {
  return request.post('/v1/users', data)
}

// 更新
export const updateAdmin = (id: number, data: any) => {
  return request.patch(`/v1/users/${id}`, data)
}

// 删除
export const deleteAdmin = (id: number) => {
  return request.delete(`/v1/users/${id}`)
}