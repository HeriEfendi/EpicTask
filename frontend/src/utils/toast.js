import { useToast } from 'vue-toastification';

/**
 * Toast Notification Utility
 * Provides convenient helpers for success, error, warning, and info notifications
 */
export const toast = {
  success(message, options = {}) {
    const t = useToast();
    return t.success(message, options);
  },
  error(message, options = {}) {
    const t = useToast();
    return t.error(message, options);
  },
  info(message, options = {}) {
    const t = useToast();
    return t.info(message, options);
  },
  warning(message, options = {}) {
    const t = useToast();
    return t.warning(message, options);
  },
  clear() {
    const t = useToast();
    return t.clear();
  }
};

export default toast;
