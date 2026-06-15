import request from '@/utils/request';

export const loginApi = (data: any) => {
  return request.post('/login', data);
};

export const getUserInfoApi = () => {
  return request.get('/user/info');
};