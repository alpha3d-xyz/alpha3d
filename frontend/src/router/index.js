import { createRouter, createWebHistory } from 'vue-router';
import Login from '../views/Login.vue';
import Signup from '../views/Signup.vue';
import { useAuthStore } from '../stores/auth';

const routes = [
  { path: '/login', component: Login },
  { path: '/signup', component: Signup },
  { path: '/', component: { template: '<div>Home Page</div>' } }, // Placeholder
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

router.beforeEach(async (to, from, next) => {
  const authStore = useAuthStore();
  if (!authStore.user && authStore.token) {
    await authStore.fetchUser();
  }
  next();
});

export default router;
