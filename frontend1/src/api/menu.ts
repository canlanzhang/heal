import request from '@/utils/request';

export const listMenus = () => {
  return request.get("/v1/menus");
};

export const createMenu = (data: any) => {
  return request.post("/v1/menus", data);
};

export const updateMenu = (id: number, data: any) => {
  return request.patch(`/v1/menus/${id}`, data);
};

export const deleteMenu = (id: number) => {
  return request.delete(`/v1/menus/${id}`);
};

export const getMenuById = (id: number) => {
  return request.get(`/v1/menus/${id}`);
};