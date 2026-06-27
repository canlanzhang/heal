import request from '@/utils/request'

// 获取列表
export const getArticleList = () => {
  return request.get('/v1/articles')
}

// 详情
export const getArticleById = (id: number) => {
  return request.get(`/v1/articles/${id}`)
}

// 新增
export const createArticle = (data: any) => {
  return request.post('/v1/articles', data)
}

// 更新
export const updateArticle = (id: number, data: any) => {
  return request.patch(`/v1/articles/${id}`, data)
}

// 删除
export const deleteArticle = (id: number) => {
  return request.delete(`/v1/articles/${id}`)
}