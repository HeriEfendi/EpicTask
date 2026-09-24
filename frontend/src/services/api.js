const API_BASE = '/api';

export class ApiError extends Error {
  constructor(message, status, data) {
    super(message);
    this.status = status;
    this.data = data;
  }
}

async function request(endpoint, options = {}) {
  const token = localStorage.getItem('epictask_token');
  const headers = {
    'Content-Type': 'application/json',
    ...options.headers,
  };

  if (token) {
    headers['Authorization'] = `Bearer ${token}`;
  }

  const url = `${API_BASE}${endpoint}`;
  const response = await fetch(url, {
    ...options,
    headers,
  });

  const contentType = response.headers.get('content-type');
  const isJson = contentType && contentType.includes('application/json');
  const data = isJson ? await response.json() : await response.text();

  if (!response.ok) {
    const message = (data && data.error) || response.statusText || 'An error occurred';
    throw new ApiError(message, response.status, data);
  }

  return data;
}

export const api = {
  get: (endpoint, query) => {
    let url = endpoint;
    if (query) {
      const q = new URLSearchParams();
      Object.entries(query).forEach(([k, v]) => {
        if (v !== undefined && v !== null && v !== '') {
          q.append(k, v);
        }
      });
      const qs = q.toString();
      if (qs) url += `?${qs}`;
    }
    return request(url, { method: 'GET' });
  },

  post: (endpoint, body) => {
    return request(endpoint, {
      method: 'POST',
      body: JSON.stringify(body || {}),
    });
  },

  put: (endpoint, body) => {
    return request(endpoint, {
      method: 'PUT',
      body: JSON.stringify(body || {}),
    });
  },

  delete: (endpoint) => {
    return request(endpoint, { method: 'DELETE' });
  },
};
