import request from '@/utils/request';

// token拦截


export interface Article {
  id: number;
  title: string;
  content: string;
  status: string;
  author_id?: number;
}

export const getArticleList = () => {
  return request.get('/articles');
};

export const getArticleById = (id: number) => {
  return request.get(`/articles/${id}`);
};

export const createArticle = (data: any) => {
  return request.post('/articles', data);
};

export const updateArticle = (id: number, data: any) => {
  return request.patch(`/articles/${id}`, data);
};

export const deleteArticle = (id: number) => {
  return request.delete(`/articles/${id}`);
};