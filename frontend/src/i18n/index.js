import { createI18n } from 'vue-i18n'
import en from '../locales/en'
import ko from '../locales/ko'

const messages = {
  en,
  ko
}

const i18n = createI18n({
  legacy: false,
  locale: 'en', // Default to English
  fallbackLocale: 'en',
  messages
})

export async function setLanguageBasedOnIP() {
  try {
    // Check if language was already manually set by user (if we had a selector)
    // For now, we just check IP
    
    const response = await fetch('https://ipapi.co/json/');
    if (response.ok) {
      const data = await response.json();
      // ipapi.co returns 'country_code' like 'KR'
      if (data.country_code === 'KR') {
        i18n.global.locale.value = 'ko';
        document.documentElement.lang = 'ko';
      }
    }
  } catch (error) {
    console.warn('Failed to detect user location, staying with default language:', error);
  }
}

export default i18n
