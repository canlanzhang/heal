import request from '@/utils/request';

//import request from './request'

export const getArticleList = () => {
  return request.get('/system/articles')
}

export const getArticleById = (id: number) => {
  return request.get(`/system/articles/${id}`)
}

export const createArticle = (data: any) => {
  return request.post('/system/articles', data)
}

export const updateArticle = (id: number, data: any) => {
  return request.put(`/system/articles/${id}`, data)
}

export const deleteArticle = (id: number) => {
  return request.delete(`/system/articles/${id}`)
}