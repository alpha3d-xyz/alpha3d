<template>
  <section class="card">
    <h2>Echo playground</h2>
    <p>Send a message to the Axum backend and see it reflected below.</p>

    <form class="form" @submit.prevent="sendEcho">
      <label for="message">Message</label>
      <input
        id="message"
        v-model="message"
        maxlength="120"
        placeholder="Hello from Vue!"
        required
      />
      <button type="submit" :disabled="sending">
        {{ sending ? 'Sending…' : 'Send to /api/echo' }}
      </button>
    </form>

    <p v-if="response" class="response">Server replied: {{ response }}</p>
    <p v-if="error" class="error">{{ error }}</p>
  </section>
</template>

<script setup>
import { ref } from 'vue';
import { apiFetch } from '../lib/apiClient.js';

const message = ref('Hello from the browser!');
const response = ref('');
const error = ref('');
const sending = ref(false);

const sendEcho = async () => {
  if (!message.value.trim()) {
    return;
  }

  sending.value = true;
  error.value = '';

  try {
    const res = await apiFetch('/echo', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ message: message.value })
    });

    if (!res.ok) {
      throw new Error('Echo failed');
    }

    const payload = await res.json();
    response.value = payload.message;
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Unknown error';
  } finally {
    sending.value = false;
  }
};
</script>

<style scoped>
.card {
  background: rgba(15, 23, 42, 0.85);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 1.25rem;
  padding: 2rem;
  box-shadow: 0 30px 80px rgba(2, 6, 23, 0.65);
}

h2 {
  margin-top: 0;
  margin-bottom: 0.5rem;
}

.form {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  margin-top: 1rem;
}

input {
  padding: 0.75rem 1rem;
  border-radius: 0.75rem;
  border: 1px solid rgba(255, 255, 255, 0.15);
  background: rgba(15, 23, 42, 0.6);
  color: #f8fafc;
}

button {
  padding: 0.85rem 1.25rem;
  border-radius: 0.75rem;
  border: none;
  font-weight: 600;
  background: linear-gradient(120deg, #38bdf8, #6366f1);
  color: #0f172a;
  cursor: pointer;
}

button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.response {
  margin-top: 1.5rem;
  font-weight: 600;
}

.error {
  margin-top: 1rem;
  color: #fca5a5;
  font-weight: 600;
}
</style>
