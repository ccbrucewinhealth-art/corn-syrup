import axios, { AxiosInstance, AxiosRequestConfig } from 'axios';
import { apiBase } from './config';

export class ApiClient {
  private client: AxiosInstance;
  private token: string | null = null;

  constructor(baseURL: string = apiBase) {
    this.client = axios.create({
      baseURL,
      timeout: 10000,
      headers: { 'Content-Type': 'application/json' },
    });
    
    this.client.interceptors.request.use((config) => {
      const fullUrl = `${apiBase}${config.url}`;
      console.log('[API Request]', {
        method: config.method?.toUpperCase(),
        url: fullUrl,
        params: config.params,
        data: config.data,
        headers: config.headers,
      });
      if (this.token) {
        config.headers.Authorization = `Bearer ${this.token}`;
      }
      return config;
    });
    
    this.client.interceptors.response.use(
      (response) => {
        console.log('[API Response]', {
          status: response.status,
          statusText: response.statusText,
          data: response.data,
        });
        return response;
      },
      (error) => {
        console.error('[API Error]', {
          message: error.message,
          status: error.response?.status,
          statusText: error.response?.statusText,
          data: error.response?.data,
        });
        return Promise.reject(error);
      }
    );
  }

  setToken(token: string | null) {
    this.token = token;
  }

  async getStatusPages() {
    const { data } = await this.client.get('/api/status-page');
    return data;
  }

  async getStatusPage(slug: string) {
    const { data } = await this.client.get(`/api/status-page/${slug}`);
    return data;
  }

  async createStatusPage(payload: Record<string, unknown>) {
    const { data } = await this.client.post('/api/status-page', payload);
    return data;
  }

  async updateStatusPage(id: number, payload: Record<string, unknown>) {
    const { data } = await this.client.put(`/api/status-page/${id}`, payload);
    return data;
  }

  async deleteStatusPage(id: number) {
    const { data } = await this.client.delete(`/api/status-page/${id}`);
    return data;
  }

  async getSettings() {
    const { data } = await this.client.get('/api/settings');
    return data;
  }

  async updateSettings(payload: Record<string, unknown>) {
    const { data } = await this.client.put('/api/settings', payload);
    return data;
  }

  async getMonitors() {
    const { data } = await this.client.get('/api/monitors');
    return data;
  }

  async getMonitor(id: number) {
    const { data } = await this.client.get(`/api/monitor/${id}`);
    return data;
  }

  async createMonitor(payload: Record<string, unknown>) {
    const { data } = await this.client.post('/api/monitor', payload);
    return data;
  }

  async updateMonitor(id: number, payload: Record<string, unknown>) {
    const { data } = await this.client.put(`/api/monitor/${id}`, payload);
    return data;
  }

  async deleteMonitor(id: number) {
    const { data } = await this.client.delete(`/api/monitor/${id}`);
    return data;
  }

  async getHeartbeats(monitorId: number, limit?: number) {
    const params = limit ? { limit: String(limit) } : undefined;
    const { data } = await this.client.get(`/api/monitor/${monitorId}/heartbeat`, { params });
    return data;
  }

  async login(username: string, password: string) {
    const { data } = await this.client.post('/api/login', { username, password });
    if (data.token) {
      this.setToken(data.token);
    }
    return data;
  }

  async register(username: string, password: string, email: string) {
    const { data } = await this.client.post('/api/register', { username, password, email });
    return data;
  }

  async logout() {
    await this.client.post('/api/logout');
    this.setToken(null);
  }
}

export const apiClient = new ApiClient();