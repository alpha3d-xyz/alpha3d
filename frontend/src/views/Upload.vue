<template>
  <div class="upload-page">
    <div class="page-header">
      <h1 class="page-title">{{ $t('upload.title') }}</h1>
      <p class="page-subtitle">{{ $t('upload.subtitle') }}</p>
    </div>

    <div class="upload-container">
      <div 
        class="drop-zone" 
        :class="{ 'is-dragging': isDragging }"
        @dragover.prevent="isDragging = true"
        @dragleave.prevent="isDragging = false"
        @drop.prevent="handleDrop"
        @click="triggerFileInput"
      >
        <input 
          type="file" 
          ref="fileInput" 
          @change="handleFileSelect" 
          accept=".stl" 
          style="display: none" 
        />
        
        <div class="drop-content">
          <div class="upload-icon">
            <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
              <polyline points="17 8 12 3 7 8"></polyline>
              <line x1="12" y1="3" x2="12" y2="15"></line>
            </svg>
          </div>
          <h3 class="drop-title">{{ $t('upload.dropTitle') }}</h3>
          <p class="drop-subtitle">{{ $t('upload.dropSubtitle') }}</p>
          <p class="file-hint">{{ $t('upload.hint') }}</p>
        </div>
      </div>

      <div v-if="fileStore.uploading" class="loading-state">
        <div class="spinner"></div>
        <p>{{ $t('upload.analyzing') }}</p>
      </div>

      <div v-if="fileStore.error" class="error-message">
        {{ fileStore.error }}
      </div>
      
      <!-- Viewer Section - Shows immediately after file selection -->
      <div v-if="fileUrl" class="viewer-section card" style="margin-bottom: 2rem;">
        <StlViewer :fileUrl="fileUrl" />
      </div>

      <div v-if="fileStore.currentFile" class="result-section">
        <h2 class="section-title">{{ $t('upload.results') }}</h2>

        <div class="result-grid">
          <div class="card result-card">
            <div class="card-header">
              <h3>{{ $t('upload.fileInfo') }}</h3>
            </div>
            <div class="card-body">
              <div class="info-row">
                <span class="label">{{ $t('upload.filename') }}:</span>
                <span class="value">{{ fileStore.currentFile.filename }}</span>
              </div>
              <div class="info-row">
                <span class="label">{{ $t('upload.id') }}:</span>
                <span class="value">{{ fileStore.currentFile.file_id }}</span>
              </div>
            </div>
          </div>

          <div class="card result-card">
            <div class="card-header">
              <h3>{{ $t('upload.geometryData') }}</h3>
            </div>
            <div class="card-body">
              <div class="stat-grid">
                <div class="stat-item">
                  <span class="stat-value">{{ formatNumber(fileStore.currentFile.volume_cm3) }}</span>
                  <span class="stat-label">{{ $t('upload.volume') }}</span>
                </div>
                <div class="stat-item">
                  <span class="stat-value">{{ formatNumber(fileStore.currentFile.surface_area_cm2) }}</span>
                  <span class="stat-label">{{ $t('upload.surfaceArea') }}</span>
                </div>
              </div>
            </div>
          </div>
        </div>
        
        <div class="actions-row">
          <button @click="clearFile" class="btn btn-secondary">{{ $t('upload.analyzeAnother') }}</button>
        </div>

        <!-- Quoting Section -->
        <div class="quoting-section">
          <h2 class="section-title">{{ $t('upload.instantQuote') }}</h2>
          <div class="quote-form card">
            <div class="form-group">
              <label>{{ $t('upload.material') }}</label>
              <select v-model="quoteForm.material" class="form-select">
                <option value="PLA">PLA (Standard)</option>
                <option value="ABS">ABS (Durable)</option>
                <option value="RESIN">Resin (High Detail)</option>
              </select>
            </div>
            <div class="form-group">
              <label>{{ $t('upload.color') }}</label>
              <select v-model="quoteForm.color" class="form-select">
                <option value="White">White</option>
                <option value="Black">Black</option>
                <option value="Grey">Grey</option>
                <option value="Red">Red</option>
                <option value="Blue">Blue</option>
              </select>
            </div>
            <div class="form-group">
              <label>{{ $t('upload.infill') }}</label>
              <input type="number" v-model="quoteForm.infill" min="10" max="100" step="10" class="form-input">
            </div>
            
            <button @click="calculateQuote" class="btn btn-primary full-width" :disabled="calculating">
              {{ calculating ? $t('upload.calculating') : $t('upload.calculatePrice') }}
            </button>
          </div>

          <div v-if="quoteResult" class="quote-result card">
            <div class="price-display">
              <span class="currency">{{ quoteResult.currency }}</span>
              <span class="amount">{{ formatNumber(quoteResult.estimated_cost) }}</span>
            </div>
            <div class="breakdown">
              <div class="breakdown-row">
                <span>{{ $t('upload.materialCost') }}</span>
                <span>{{ formatNumber(quoteResult.breakdown.material_cost) }}</span>
              </div>
              <div class="breakdown-row">
                <span>{{ $t('upload.machineCost') }}</span>
                <span>{{ formatNumber(quoteResult.breakdown.machine_cost) }}</span>
              </div>
            </div>
            <button @click="placeOrder" class="btn btn-success full-width" :disabled="ordering">
              {{ ordering ? $t('upload.processing') : $t('upload.placeOrder') }}
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive } from 'vue';
import { useFileStore } from '../stores/files';
import { apiClient } from '../lib/apiClient';
import { useRouter } from 'vue-router';
import StlViewer from '../components/StlViewer.vue';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();
const router = useRouter();
const fileStore = useFileStore();
const fileInput = ref(null);
const calculating = ref(false);
const ordering = ref(false);
const quoteResult = ref(null);
const fileUrl = ref(null);

const formatNumber = (num) => {
  return new Intl.NumberFormat('en-US').format(num);
};

const quoteForm = reactive({
  material: 'PLA',
  color: 'White',
  infill: 20
});

const clearFile = () => {
  fileStore.clearFile();
  if (fileUrl.value) {
    URL.revokeObjectURL(fileUrl.value);
    fileUrl.value = null;
  }
  quoteResult.value = null;
};

const calculateQuote = async () => {
  if (!fileStore.currentFile) return;
  
  calculating.value = true;
  try {
    const response = await apiClient.post('/api/quotes/calculate', {
      file_id: fileStore.currentFile.file_id,
      material: quoteForm.material,
      color: quoteForm.color,
      infill_percentage: quoteForm.infill
    });
    quoteResult.value = response.data;
  } catch (error) {
    console.error('Quote calculation failed:', error);
    alert(t('upload.failedQuote'));
  } finally {
    calculating.value = false;
  }
};

const placeOrder = async () => {
  if (!quoteResult.value) return;
  
  ordering.value = true;
  try {
    // Mock shipping address for now
    const shippingAddress = {
      recipient: "Demo User",
      address: "123 Maker Street",
      city: "Seoul",
      zip: "04524"
    };

    const response = await apiClient.post('/api/orders', {
      quote_id: quoteResult.value.id,
      shipping_address: shippingAddress
    });
    
    alert(t('upload.orderPlaced', { id: response.data.id }));
    // router.push('/orders'); // TODO: Implement orders page
  } catch (error) {
    console.error('Order placement failed:', error);
    alert(t('upload.failedOrder'));
  } finally {
    ordering.value = false;
  }
};

const isDragging = ref(false);

const triggerFileInput = () => {
  fileInput.value.click();
};

const handleFileSelect = (event) => {
  const file = event.target.files[0];
  if (file) processFile(file);
};

const handleDrop = (event) => {
  isDragging.value = false;
  const file = event.dataTransfer.files[0];
  if (file) processFile(file);
};

const processFile = async (file) => {
  if (!file.name.toLowerCase().endsWith('.stl')) {
    alert(t('upload.invalidFile'));
    return;
  }
  
  // Create object URL for viewer
  if (fileUrl.value) URL.revokeObjectURL(fileUrl.value);
  fileUrl.value = URL.createObjectURL(file);
  
  console.log('[Upload] Created blob URL for viewer:', fileUrl.value);
  console.log('[Upload] File details:', { name: file.name, size: file.size, type: file.type });
  
  await fileStore.uploadFile(file);
};
</script>

<style scoped>
.upload-page {
  max-width: 1000px;
  margin: 0 auto;
}

.page-header {
  text-align: center;
  margin-bottom: 2.5rem;
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

.drop-zone {
  border: 2px dashed var(--border-color);
  border-radius: var(--radius-lg);
  padding: 4rem 2rem;
  text-align: center;
  background-color: var(--bg-card);
  cursor: pointer;
  transition: all 0.2s ease;
  margin-bottom: 2rem;
}

.drop-zone:hover, .drop-zone.is-dragging {
  border-color: var(--primary-color);
  background-color: rgba(79, 70, 229, 0.05);
}

.upload-icon {
  color: var(--primary-color);
  margin-bottom: 1.5rem;
}

.drop-title {
  font-size: 1.25rem;
  font-weight: 600;
  margin-bottom: 0.5rem;
  color: var(--text-primary);
}

.drop-subtitle {
  color: var(--text-secondary);
  margin-bottom: 0.5rem;
}

.file-hint {
  font-size: 0.875rem;
  color: var(--text-muted);
}

.loading-state {
  text-align: center;
  padding: 2rem;
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
  margin-bottom: 2rem;
  text-align: center;
  border: 1px solid #fee2e2;
}

.section-title {
  font-size: 1.5rem;
  font-weight: 600;
  margin-bottom: 1.5rem;
  color: var(--text-primary);
}

.result-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 1.5rem;
  margin-bottom: 2rem;
}

.result-card {
  height: 100%;
}

.card-header {
  padding-bottom: 1rem;
  border-bottom: 1px solid var(--border-color);
  margin-bottom: 1rem;
}

.card-header h3 {
  font-size: 1.1rem;
  font-weight: 600;
  color: var(--text-primary);
}

.info-row {
  display: flex;
  justify-content: space-between;
  padding: 0.5rem 0;
  border-bottom: 1px solid var(--bg-main);
}

.info-row:last-child {
  border-bottom: none;
}

.info-row .label {
  color: var(--text-secondary);
  font-weight: 500;
}

.info-row .value {
  font-family: monospace;
  color: var(--text-primary);
}

.stat-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 1rem;
  text-align: center;
}

.stat-item {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.stat-value {
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--primary-color);
}

.stat-label {
  font-size: 0.875rem;
  color: var(--text-secondary);
}

.actions-row {
  display: flex;
  justify-content: center;
}

.viewer-section {
  margin-bottom: 2rem;
  overflow: hidden;
  border-radius: var(--radius-lg);
  border: 1px solid var(--border-color);
}

@media (max-width: 640px) {
  .page-title {
    font-size: 1.5rem;
  }
  
  .drop-zone {
    padding: 2rem 1rem;
  }
  
  .stat-grid {
    grid-template-columns: 1fr;
  }
}
</style>
