import request from '@/utils/request';
/*
 * 登录接口
 */
export const loginApi = (data: {
  username: string
  password: string
}) => {
  return request.post('/v1/auth/login', data)
}

/**
 * 获取当前用户信息 + 菜单
 */
export const getUserInfoApi = () => {
  return request.get('/v1/auth/profile')
}