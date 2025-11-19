<template>
  <div class="orders-page">
    <div class="page-header">
      <h1 class="page-title">Order History</h1>
      <p class="page-subtitle">Track your 3D printing orders</p>
    </div>

    <div v-if="loading" class="loading-state">
      <div class="spinner"></div>
      <p>Loading orders...</p>
    </div>

    <div v-else-if="error" class="error-message">
      {{ error }}
    </div>

    <div v-else-if="orders.length === 0" class="empty-state">
      <div class="empty-icon">
        <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <rect x="3" y="8" width="18" height="13" rx="2" ry="2"></rect>
          <polyline points="3 8 12 13 21 8"></polyline>
        </svg>
      </div>
      <h3>No orders yet</h3>
      <p>Your order history will appear here once you place your first order.</p>
      <router-link to="/" class="btn btn-primary">Upload a File</router-link>
    </div>

    <div v-else class="orders-container">
      <div class="order-card card" v-for="order in orders" :key="order.id">
        <div class="order-header">
          <div class="order-info">
            <h3 class="order-id">Order #{{ order.id.substring(0, 8) }}</h3>
            <span class="order-date">{{ formatDate(order.created_at) }}</span>
          </div>
          <span class="status-badge" :class="`status-${order.status.toLowerCase()}`">
            {{ order.status }}
          </span>
        </div>
        
        <div class="order-body">
          <div class="order-detail">
            <span class="label">Order ID:</span>
            <span class="value">{{ order.id }}</span>
          </div>
          <div class="order-detail">
            <span class="label">Status:</span>
            <span class="value">{{ getStatusDescription(order.status) }}</span>
          </div>
          <div class="order-detail">
            <span class="label">Created:</span>
            <span class="value">{{ formatDateTime(order.created_at) }}</span>
          </div>
        </div>

        <div class="order-actions">
          <button @click="viewOrderDetails(order.id)" class="btn btn-outline btn-sm">
            View Details
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { apiClient } from '../lib/apiClient';
import { useRouter } from 'vue-router';

const router = useRouter();
const orders = ref([]);
const loading = ref(true);
const error = ref(null);

const fetchOrders = async () => {
  loading.value = true;
  error.value = null;
  
  try {
    const response = await apiClient.get('/api/orders');
    orders.value = response.data;
  } catch (err) {
    console.error('Failed to fetch orders:', err);
    error.value = 'Failed to load orders. Please try again.';
  } finally {
    loading.value = false;
  }
};

const formatDate = (dateString) => {
  const date = new Date(dateString);
  return date.toLocaleDateString('en-US', { 
    year: 'numeric', 
    month: 'short', 
    day: 'numeric' 
  });
};

const formatDateTime = (dateString) => {
  const date = new Date(dateString);
  return date.toLocaleString('en-US', { 
    year: 'numeric', 
    month: 'short', 
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  });
};

const getStatusDescription = (status) => {
  const statusMap = {
    'PENDING': 'Awaiting payment confirmation',
    'PAID': 'Payment received, preparing to print',
    'PRINTING': 'Currently being printed',
    'SHIPPED': 'Shipped and on the way',
    'CANCELLED': 'Order cancelled'
  };
  return statusMap[status] || status;
};

const viewOrderDetails = (orderId) => {
  // TODO: Implement order details modal or page
  alert(`Order details for ${orderId} - Feature coming soon!`);
};

onMounted(() => {
  fetchOrders();
});
</script>

<style scoped>
.orders-page {
  max-width: 1200px;
  margin: 0 auto;
  padding: 2rem 1rem;
}

.page-header {
  text-align: center;
  margin-bottom: 3rem;
}

.page-title {
  font-size: 2rem;
  font-weight: 700;
  color: var(--text-primary);
  margin-bottom: 0.5rem;
}

.page-subtitle {
  color: var(--text-secondary);
  font-size: 1.1rem;
}

.loading-state {
  text-align: center;
  padding: 4rem 2rem;
  color: var(--text-secondary);
}

.spinner {
  width: 40px;
  height: 40px;
  border: 3px solid rgba(79, 70, 229, 0.1);
  border-radius: 50%;
  border-top-color: var(--primary-color);
  animation: spin 1s ease-in-out infinite;
  margin: 0 auto 1rem;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.error-message {
  background-color: #fef2f2;
  color: var(--error-color);
  padding: 1rem;
  border-radius: var(--radius-md);
  text-align: center;
  border: 1px solid #fee2e2;
  margin: 2rem 0;
}

.empty-state {
  text-align: center;
  padding: 4rem 2rem;
}

.empty-icon {
  color: var(--text-secondary);
  margin-bottom: 1.5rem;
  opacity: 0.5;
}

.empty-state h3 {
  font-size: 1.5rem;
  color: var(--text-primary);
  margin-bottom: 0.5rem;
}

.empty-state p {
  color: var(--text-secondary);
  margin-bottom: 2rem;
}

.orders-container {
  display: grid;
  gap: 1.5rem;
}

.order-card {
  padding: 1.5rem;
  transition: transform 0.2s, box-shadow 0.2s;
}

.order-card:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-lg);
}

.order-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 1rem;
  padding-bottom: 1rem;
  border-bottom: 1px solid var(--border-color);
}

.order-info {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.order-id {
  font-size: 1.1rem;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.order-date {
  font-size: 0.875rem;
  color: var(--text-secondary);
}

.status-badge {
  padding: 0.25rem 0.75rem;
  border-radius: 9999px;
  font-size: 0.75rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.status-pending {
  background-color: #fef3c7;
  color: #92400e;
}

.status-paid {
  background-color: #dbeafe;
  color: #1e40af;
}

.status-printing {
  background-color: #e0e7ff;
  color: #4338ca;
}

.status-shipped {
  background-color: #d1fae5;
  color: #065f46;
}

.status-cancelled {
  background-color: #fee2e2;
  color: #991b1b;
}

.order-body {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  margin-bottom: 1rem;
}

.order-detail {
  display: flex;
  justify-content: space-between;
  font-size: 0.875rem;
}

.order-detail .label {
  color: var(--text-secondary);
  font-weight: 500;
}

.order-detail .value {
  color: var(--text-primary);
  font-family: monospace;
  font-size: 0.8rem;
}

.order-actions {
  display: flex;
  gap: 0.5rem;
  justify-content: flex-end;
}

.btn-sm {
  padding: 0.375rem 0.75rem;
  font-size: 0.875rem;
}

@media (max-width: 640px) {
  .page-title {
    font-size: 1.5rem;
  }
  
  .order-header {
    flex-direction: column;
    gap: 0.75rem;
  }
  
  .order-detail {
    flex-direction: column;
    gap: 0.25rem;
  }
}
</style>
