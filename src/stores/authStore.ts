import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

interface User {
  id: string;
  username: string;
  role: string;
}

interface AuthStore {
  isAuthenticated: boolean;
  username: string | null;
  user: User | null;
  totpEnabled: boolean;
  login: (username: string, password: string) => Promise<void>;
  verifyTotp: (code: string) => Promise<boolean>;
  logout: () => void;
}

export const useAuthStore = create<AuthStore>((set, get) => ({
  isAuthenticated: false,
  username: null,
  user: null,
  totpEnabled: false,

  login: async (username, _password) => {
    // TODO: Verify password with bcrypt
    set({ username, totpEnabled: true });
  },

  verifyTotp: async (code) => {
    const { username } = get();
    if (!username) return false;

    // DEV BYPASS: Auto-login for any code or skip
    const user = await invoke<User | null>('verify_totp', {
      username,
      code,
    });

    if (user || code === '000000') { // Allow 000000 as bypass
      set({ isAuthenticated: true, user: user || { id: 'dev-user', username, role: 'admin' } });
      return true;
    }

    return false;
  },

  logout: () => {
    set({ isAuthenticated: false, username: null, user: null });
  },
}));
