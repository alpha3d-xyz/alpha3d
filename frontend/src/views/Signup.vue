<template>
  <div class="auth-wrapper">
    <div class="card auth-card">
      <h2 class="auth-title">Create Account</h2>
      <p class="auth-subtitle">Get started with Alpha3D</p>
      
      <form @submit.prevent="handleSignup" class="auth-form">
        <div class="form-group">
          <label class="form-label">Email</label>
          <input v-model="email" type="email" required class="input-field" placeholder="name@example.com" />
        </div>
        <div class="form-group">
          <label class="form-label">Password</label>
          <input v-model="password" type="password" required class="input-field" placeholder="••••••••" />
        </div>
        
        <button type="submit" :disabled="authStore.loading" class="btn btn-primary full-width">
          {{ authStore.loading ? 'Creating account...' : 'Sign Up' }}
        </button>
        
        <div v-if="authStore.error" class="error-message">
          {{ authStore.error }}
        </div>
      </form>
      
      <div class="auth-footer">
        <p>
          Already have an account? <router-link to="/login">Sign in</router-link>
        </p>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { useAuthStore } from '../stores/auth';
import { useRouter } from 'vue-router';

const email = ref('');
const password = ref('');
const authStore = useAuthStore();
const router = useRouter();

const handleSignup = async () => {
  const success = await authStore.signup(email.value, password.value);
  if (success) {
    alert('Signup successful! Please login.');
    router.push('/login');
  }
};
</script>

<style scoped>
.auth-wrapper {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: calc(100vh - 200px);
}

.auth-card {
  width: 100%;
  max-width: 400px;
}

.auth-title {
  text-align: center;
  margin-bottom: 0.5rem;
  font-size: 1.5rem;
}

.auth-subtitle {
  text-align: center;
  color: var(--text-secondary);
  margin-bottom: 2rem;
  font-size: 0.875rem;
}

.auth-form {
  display: flex;
  flex-direction: column;
  gap: 1.25rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.form-label {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--text-primary);
}

.full-width {
  width: 100%;
  margin-top: 0.5rem;
}

.error-message {
  color: var(--error-color);
  font-size: 0.875rem;
  text-align: center;
  background-color: #fef2f2;
  padding: 0.75rem;
  border-radius: var(--radius-sm);
  border: 1px solid #fee2e2;
}

.auth-footer {
  margin-top: 1.5rem;
  text-align: center;
  font-size: 0.875rem;
  color: var(--text-secondary);
}

.auth-footer a {
  font-weight: 500;
}
</style>
