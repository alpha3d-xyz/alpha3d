const rawBase = import.meta.env.VITE_API_BASE_URL ?? '/api';
const normalizedBase = rawBase.endsWith('/') ? rawBase.slice(0, -1) : rawBase;

const buildPath = (path) => {
  if (path.startsWith('/')) {
    return `${normalizedBase}${path}`;
  }
  return `${normalizedBase}/${path}`;
};

export const apiFetch = (path, options) => fetch(buildPath(path), options);
