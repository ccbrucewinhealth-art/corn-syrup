import { io, Socket } from 'socket.io-client';
import { getWsUrl } from './config';

export type SocketEventCallback = (data: unknown) => void;

export class SocketClient {
  private socket: Socket | null = null;
  private listeners: Map<string, Set<SocketEventCallback>> = new Map();

  connect(token?: string) {
    if (this.socket?.connected) return;
    
    const wsUrl = getWsUrl();
    console.log('[Socket] Connecting to', wsUrl);
    const opts: Record<string, unknown> = {
      transports: ['websocket'],
      autoConnect: true,
    };
    if (token) {
      opts.auth = { token };
    }
    
    this.socket = io(wsUrl, opts);
    
    this.socket.on('connect', () => {
      console.log('[Socket] Connected');
    });
    
    this.socket.on('disconnect', (reason) => {
      console.log('[Socket] Disconnected:', reason);
    });
    
    this.socket.on('connect_error', (error) => {
      console.error('[Socket] Connect error:', error.message);
    });
    
    this.socket.onAny((event, ...args) => {
      console.log('[Socket] Event:', event, args);
    });
  }

  disconnect() {
    if (this.socket) {
      this.socket.disconnect();
      this.socket = null;
    }
  }

  on(event: string, callback: SocketEventCallback) {
    if (!this.listeners.has(event)) {
      this.listeners.set(event, new Set());
    }
    this.listeners.get(event)!.add(callback);
    this.socket?.on(event, callback);
  }

  off(event: string, callback?: SocketEventCallback) {
    if (callback) {
      this.listeners.get(event)?.delete(callback);
      this.socket?.off(event, callback);
    } else {
      this.listeners.delete(event);
      this.socket?.off(event);
    }
  }

  emit(event: string, data?: unknown) {
    console.log('[Socket] Emit:', { event, data });
    if (!this.socket?.connected) {
      console.warn('[Socket] Not connected, emit skipped');
      return;
    }
    this.socket.emit(event, data);
  }

  getSocket() {
    return this.socket;
  }

  isConnected() {
    return this.socket?.connected ?? false;
  }
}

export const socketClient = new SocketClient();