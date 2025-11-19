<template>
  <div class="app-layout">
    <header class="navbar">
      <div class="container nav-content">
        <div class="logo">Alpha3D</div>
        <nav class="nav-links">
          <template v-if="authStore.isAuthenticated">
            <router-link to="/" class="nav-item">{{ $t('nav.upload') }}</router-link>
            <router-link to="/orders" class="nav-item">{{ $t('nav.orders') }}</router-link>
            <span class="user-email">{{ authStore.user?.email }}</span>
            <button @click="handleLogout" class="btn btn-outline btn-sm">{{ $t('nav.logout') }}</button>
          </template>
          <template v-else>
            <router-link to="/login" class="nav-item">{{ $t('nav.login') }}</router-link>
            <router-link to="/signup" class="btn btn-primary btn-sm">{{ $t('nav.signup') }}</router-link>
          </template>
        </nav>
      </div>
    </header>
    <main class="main-content container">
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
.app-layout {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

.navbar {
  background-color: var(--surface-color);
  border-bottom: 1px solid var(--border-color);
  padding: 1rem 0;
  position: sticky;
  top: 0;
  z-index: 10;
}

.nav-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.logo {
  font-weight: 700;
  font-size: 1.25rem;
  color: var(--primary-color);
  letter-spacing: -0.025em;
}

.nav-links {
  display: flex;
  align-items: center;
  gap: 1.5rem;
}

.nav-item {
  color: var(--text-secondary);
  font-weight: 500;
  font-size: 0.875rem;
}

.nav-item:hover, .nav-item.router-link-active {
  color: var(--text-primary);
}

.user-email {
  font-size: 0.875rem;
  color: var(--text-secondary);
  display: none; /* Hidden on mobile by default */
}

.btn-sm {
  padding: 0.375rem 0.75rem;
  font-size: 0.875rem;
}

.main-content {
  flex: 1;
  padding-top: 2rem;
  padding-bottom: 2rem;
}

@media (min-width: 640px) {
  .user-email {
    display: inline-block;
  }
}
</style>
