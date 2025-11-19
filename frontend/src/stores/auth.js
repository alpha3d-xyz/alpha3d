import { defineStore } from 'pinia';
import axios from 'axios';

export const useAuthStore = defineStore('auth', {
  state: () => ({
    user: null,
    token: localStorage.getItem('token') || null,
    loading: false,
    error: null,
  }),
  getters: {
    isAuthenticated: (state) => !!state.token,
  },
  actions: {
    async login(email, password) {
      this.loading = true;
      this.error = null;
      try {
        const response = await axios.post('/api/auth/login', { email, password });
        this.token = response.data.token;
        localStorage.setItem('token', this.token);
        await this.fetchUser();
        return true;
      } catch (err) {
        this.error = err.response?.data || 'Login failed';
        return false;
      } finally {
        this.loading = false;
      }
    },
    async signup(email, password) {
      this.loading = true;
      this.error = null;
      try {
        await axios.post('/api/auth/signup', { email, password });
        return true;
      } catch (err) {
        this.error = err.response?.data || 'Signup failed';
        return false;
      } finally {
        this.loading = false;
      }
    },
    async fetchUser() {
      if (!this.token) return;
      try {
        const response = await axios.get('/api/auth/me', {
          headers: { Authorization: `Bearer ${this.token}` },
        });
        this.user = response.data;
      } catch (err) {
        this.logout();
      }
    },
    logout() {
      this.user = null;
      this.token = null;
      localStorage.removeItem('token');
    },
  },
});
