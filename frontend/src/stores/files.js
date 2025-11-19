import { defineStore } from 'pinia';
import axios from 'axios';
import { useAuthStore } from './auth';

export const useFileStore = defineStore('files', {
  state: () => ({
    currentFile: null,
    uploading: false,
    error: null,
  }),
  actions: {
    async uploadFile(file) {
      this.uploading = true;
      this.error = null;
      const authStore = useAuthStore();
      
      const formData = new FormData();
      formData.append('file', file);

      try {
        const response = await axios.post('/api/files/upload', formData, {
          headers: {
            'Content-Type': 'multipart/form-data',
            Authorization: `Bearer ${authStore.token}`,
          },
        });
        this.currentFile = response.data;
        return true;
      } catch (err) {
        this.error = err.response?.data || 'Upload failed';
        return false;
      } finally {
        this.uploading = false;
      }
    },
    clearFile() {
      this.currentFile = null;
      this.error = null;
    }
  },
});
