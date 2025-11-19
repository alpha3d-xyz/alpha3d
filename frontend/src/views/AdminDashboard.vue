<template>
  <div class="admin-dashboard">
    <div class="page-header">
      <h1 class="page-title">Admin Dashboard</h1>
      <p class="page-subtitle">Manage orders and system status</p>
    </div>

    <div v-if="loading" class="loading-state">Loading orders...</div>
    <div v-else-if="error" class="error-message">{{ error }}</div>
    
    <div v-else class="orders-list">
      <div v-for="order in orders" :key="order.id" class="card order-card">
        <div class="order-header">
          <span class="order-id">Order #{{ order.id.slice(0, 8) }}</span>
          <span class="order-date">{{ new Date(order.created_at).toLocaleDateString() }}</span>
        </div>
        
        <div class="order-details">
          <div class="detail-row">
            <span class="label">User:</span>
            <span class="value">{{ order.user_email }}</span>
          </div>
          <div class="detail-row">
            <span class="label">Material:</span>
            <span class="value">{{ order.material }}</span>
          </div>
          <div class="detail-row">
            <span class="label">Cost:</span>
            <span class="value">${{ order.estimated_cost.toFixed(2) }}</span>
          </div>
          <div class="detail-row">
            <span class="label">Status:</span>
            <select 
              :value="order.status" 
              @change="updateStatus(order.id, $event.target.value)"
              class="status-select"
              :class="order.status.toLowerCase()"
            >
              <option value="PENDING">PENDING</option>
              <option value="PAID">PAID</option>
              <option value="PRINTING">PRINTING</option>
              <option value="SHIPPED">SHIPPED</option>
              <option value="CANCELLED">CANCELLED</option>
            </select>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { apiFetch } from '../lib/apiClient';

const orders = ref([]);
const loading = ref(true);
const error = ref(null);

const fetchOrders = async () => {
  try {
    loading.value = true;
    const res = await apiFetch('/admin/orders');
    if (!res.ok) throw new Error('Failed to fetch orders');
    orders.value = await res.json();
  } catch (err) {
    error.value = err.message;
  } finally {
    loading.value = false;
  }
};

const updateStatus = async (orderId, newStatus) => {
  try {
    const res = await apiFetch(`/admin/orders/${orderId}/status`, {
      method: 'PATCH',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ status: newStatus })
    });
    
    if (!res.ok) throw new Error('Failed to update status');
    
    // Update local state
    const order = orders.value.find(o => o.id === orderId);
    if (order) order.status = newStatus;
    
  } catch (err) {
    alert('Failed to update status: ' + err.message);
  }
};

onMounted(fetchOrders);
</script>

<style scoped>
.admin-dashboard {
  max-width: 800px;
  margin: 0 auto;
  padding: 2rem 0;
}

.page-header {
  margin-bottom: 2rem;
}

.page-title {
  font-size: 2rem;
  font-weight: 700;
  color: var(--text-primary);
  margin-bottom: 0.5rem;
}

.page-subtitle {
  color: var(--text-secondary);
}

.orders-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.order-card {
  padding: 1.5rem;
}

.order-header {
  display: flex;
  justify-content: space-between;
  margin-bottom: 1rem;
  padding-bottom: 0.5rem;
  border-bottom: 1px solid var(--border-color);
}

.order-id {
  font-weight: 600;
  font-family: monospace;
}

.order-date {
  color: var(--text-secondary);
  font-size: 0.875rem;
}

.detail-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.5rem;
}

.label {
  color: var(--text-secondary);
}

.status-select {
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  border: 1px solid var(--border-color);
  background-color: var(--surface-color);
  color: var(--text-primary);
}

.status-select.paid { color: var(--success-color); border-color: var(--success-color); }
.status-select.printing { color: var(--primary-color); border-color: var(--primary-color); }
.status-select.shipped { color: var(--info-color); border-color: var(--info-color); }
.status-select.cancelled { color: var(--error-color); border-color: var(--error-color); }
</style>
