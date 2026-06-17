import axios from 'axios'
import { useUserStore } from '@/store/user'

const request = axios.create({
  baseURL: '/api', // 改成你的后端
  timeout: 10000
})

// 请求拦截（关键）
request.interceptors.request.use(config => {
  const store = useUserStore()

  if (store.token) {
    config.headers.Authorization = `Bearer ${store.token}`
  }

  return config
})

request.interceptors.response.use(res => {
  return res.data   // ⭐关键：去掉 Axios 外层
})


export default request