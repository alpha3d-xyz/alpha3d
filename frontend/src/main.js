import { createApp } from 'vue';
import { createPinia } from 'pinia';
import App from './App.vue';
import router from './router';
import i18n, { setLanguageBasedOnIP } from './i18n';
import './style.css';

const app = createApp(App);

app.use(createPinia());
app.use(router);
app.use(i18n);
app.mount('#app');

// Attempt to set language based on IP
// Skip in automated environments (tests) to avoid network timeouts
if (!navigator.webdriver) {
  setLanguageBasedOnIP();
}
