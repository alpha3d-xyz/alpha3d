const rawBase = import.meta.env.VITE_API_BASE_URL ?? '/api';
const normalizedBase = rawBase.endsWith('/') ? rawBase.slice(0, -1) : rawBase;

const buildPath = (path) => {
  // If path starts with /api and base also ends with /api, avoid duplication
  // But here normalizedBase is likely just empty string or a full URL.
  // If we are proxying via vite, VITE_API_BASE_URL might be empty.
  // Let's assume path passed to client methods includes /api if needed, 
  // OR the base url includes it.
  // In Upload.vue I used '/api/quotes/calculate'.
  
  // If VITE_API_BASE_URL is not set, it defaults to '/api'.
  // So '/api' + '/api/quotes/calculate' -> '/api/api/quotes/calculate' which is wrong.
  
  // Let's adjust the default base to be empty string if we expect paths to contain /api
  // OR adjust the paths in Upload.vue.
  
  // Looking at the existing code:
  // const rawBase = import.meta.env.VITE_API_BASE_URL ?? '/api';
  
  // If I change default to '', then `fetch('/api/...')` works relative to domain.
  
  if (path.startsWith('http')) return path;
  
  const cleanPath = path.startsWith('/') ? path : `/${path}`;
  
  // If base is just /api and path starts with /api, don't double it
  if (normalizedBase === '/api' && cleanPath.startsWith('/api')) {
    return cleanPath;
  }
  
  return `${normalizedBase}${cleanPath}`;
};

const request = async (method, path, data = null, customHeaders = {}) => {
  const headers = { ...customHeaders };
  
  // Only set Content-Type to application/json if data is NOT FormData
  // If data is FormData, let the browser set Content-Type (with boundary)
  if (!(data instanceof FormData)) {
    headers['Content-Type'] = 'application/json';
  }

  // Add Authorization header if token exists
  const token = localStorage.getItem('token');
  if (token) {
    headers['Authorization'] = `Bearer ${token}`;
  }

  const options = {
    method,
    headers,
  };

  if (data) {
    options.body = (data instanceof FormData) ? data : JSON.stringify(data);
  }

  const response = await fetch(buildPath(path), options);

  
  if (!response.ok) {
    const error = new Error('API request failed');
    error.response = response;
    try {
      error.data = await response.json();
    } catch (e) {
      error.data = null;
    }
    throw error;
  }

  // Return an object that mimics axios response
  const responseData = await response.json();
  return { data: responseData, status: response.status };
};

export const apiClient = {
  get: (path, headers) => request('GET', path, null, headers),
  post: (path, data, headers) => request('POST', path, data, headers),
  put: (path, data, headers) => request('PUT', path, data, headers),
  delete: (path, headers) => request('DELETE', path, null, headers),
};

export const apiFetch = (path, options) => fetch(buildPath(path), options);

