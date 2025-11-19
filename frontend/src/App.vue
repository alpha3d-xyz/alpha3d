<template>
  <div class="app-container">
    <nav>
      <div class="logo">Alpha3D</div>
      <div class="nav-links">
        <router-link to="/">Home</router-link>
        <template v-if="authStore.isAuthenticated">
          <span>Welcome, {{ authStore.user?.email }}</span>
          <button @click="handleLogout">Logout</button>
        </template>
        <template v-else>
          <router-link to="/login">Login</router-link>
          <router-link to="/signup">Signup</router-link>
        </template>
      </div>
    </nav>
    <main>
      <router-view />
    </main>
  </div>
</template>

<script setup>
import { useAuthStore } from './stores/auth';
import { useRouter } from 'vue-router';

const authStore = useAuthStore();
const router = useRouter();

const handleLogout = () => {
  authStore.logout();
  router.push('/login');
};
</script>

<style scoped>
.app-container {
  font-family: Arial, sans-serif;
}
nav {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 2rem;
  background-color: #333;
  color: white;
}
.logo {
  font-weight: bold;
  font-size: 1.2rem;
}
.nav-links a {
  color: white;
  text-decoration: none;
  margin-left: 1rem;
}
.nav-links button {
  margin-left: 1rem;
  padding: 0.5rem 1rem;
  background-color: #ff4d4d;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}
main {
  padding: 2rem;
}
</style>
