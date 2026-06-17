import request from '@/utils/request';

/**
 * 获取管理员列表（带分页）
 */
export const getAdminList = (params: any) => {
  return request.get('/system/admins', { params });
};

/**
 * 新增管理员
 */
export const createAdmin = (data: any) => {
  return request.post('/system/admins', data);
};

/**
 * 更新管理员
 */
export const updateAdmin = (id: number, data: any) => {
  return request.patch(`/system/admins/${id}`, data);
};

/**
 * 删除管理员
 */
export const deleteAdmin = (id: number) => {
  return request.delete(`/system/admins/${id}`);
};