// Tauri desktop integration service

export const isTauri = () => {
  return typeof window !== 'undefined' && (
    !!window.__TAURI_INTERNALS__ || 
    !!window.__TAURI__ || 
    !!window.__TAURI_METADATA__
  );
};

export async function sendDesktopNotification(title, body) {
  if (isTauri()) {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      await invoke('send_desktop_notification', { title, body });
      return;
    } catch (e) {
      console.warn('Tauri invoke failed:', e);
    }
  }

  // Web Notification fallback
  if ('Notification' in window && Notification.permission === 'granted') {
    new Notification(title, { body, icon: '/src/assets/logo.svg' });
  } else if ('Notification' in window && Notification.permission !== 'denied') {
    Notification.requestPermission().then((permission) => {
      if (permission === 'granted') {
        new Notification(title, { body, icon: '/src/assets/logo.svg' });
      }
    });
  }
}
