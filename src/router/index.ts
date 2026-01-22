import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'
import ProjectList from '@/components/features/ProjectList.vue'
// ProjectDetail 将在创建后导入

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'home',
    component: ProjectList,
    meta: {
      title: '项目列表'
    }
  },
  {
    path: '/projects/:id',
    name: 'project-detail',
    // 懒加载 ProjectDetail 组件
    component: () => import('@/components/features/ProjectDetail.vue'),
    meta: {
      title: '项目详情'
    }
  },
  {
    // 404 重定向
    path: '/:pathMatch(.*)*',
    redirect: '/'
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router
