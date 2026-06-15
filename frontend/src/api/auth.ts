import request from '@/utils/request';

export const loginApi = (data: any) => {
  return request.post('/system/auth/login', data);
};

export const getUserInfoApi = () => {
  return request.get('/system/auth/me');
};