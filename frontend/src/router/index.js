import { createRouter, createWebHistory } from 'vue-router';
import Login from '../views/Login.vue';
import Signup from '../views/Signup.vue';
import Upload from '../views/Upload.vue';
import { useAuthStore } from '../stores/auth';

const routes = [
  { path: '/login', component: Login },
  { path: '/signup', component: Signup },
  { path: '/', component: Upload, meta: { requiresAuth: true } },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

router.beforeEach(async (to, from, next) => {
  const authStore = useAuthStore();
  
  // Try to restore session
  if (!authStore.user && authStore.token) {
    await authStore.fetchUser();
  }

  if (to.meta.requiresAuth && !authStore.isAuthenticated) {
    next('/login');
  } else {
    next();
  }
});

export default router;
