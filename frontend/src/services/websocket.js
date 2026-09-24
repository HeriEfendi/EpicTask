class WebSocketClient {
  constructor() {
    this.ws = null;
    this.reconnectTimer = null;
    this.listeners = new Set();
    this.isConnected = false;
    this.connectionStateListeners = new Set();
  }

  connect() {
    if (this.ws && (this.ws.readyState === WebSocket.OPEN || this.ws.readyState === WebSocket.CONNECTING)) {
      return;
    }

    const token = localStorage.getItem('epictask_token');
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const host = window.location.host;
    const wsUrl = `${protocol}//${host}/ws${token ? `?token=${token}` : ''}`;

    try {
      this.ws = new WebSocket(wsUrl);

      this.ws.onopen = () => {
        this.isConnected = true;
        this.notifyState(true);
        if (this.reconnectTimer) {
          clearTimeout(this.reconnectTimer);
          this.reconnectTimer = null;
        }
      };

      this.ws.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data);
          this.listeners.forEach((listener) => listener(data));
        } catch (e) {
          console.error('[WS] Failed to parse message', e);
        }
      };

      this.ws.onclose = () => {
        this.isConnected = false;
        this.notifyState(false);
        this.scheduleReconnect();
      };

      this.ws.onerror = (err) => {
        console.warn('[WS] Socket error, reconnecting...', err);
        this.ws.close();
      };
    } catch (e) {
      this.scheduleReconnect();
    }
  }

  scheduleReconnect() {
    if (!this.reconnectTimer) {
      this.reconnectTimer = setTimeout(() => {
        this.reconnectTimer = null;
        this.connect();
      }, 3000);
    }
  }

  subscribe(listener) {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  onStateChange(listener) {
    this.connectionStateListeners.add(listener);
    listener(this.isConnected);
    return () => this.connectionStateListeners.delete(listener);
  }

  notifyState(state) {
    this.connectionStateListeners.forEach((l) => l(state));
  }

  send(data) {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      this.ws.send(typeof data === 'string' ? data : JSON.stringify(data));
    }
  }

  disconnect() {
    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer);
      this.reconnectTimer = null;
    }
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
    this.isConnected = false;
    this.notifyState(false);
  }
}

export const wsClient = new WebSocketClient();
