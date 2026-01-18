import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

interface AuthStore {
  isAuthenticated: boolean;
  username: string | null;
  totpEnabled: boolean;
  login: (username: string, password: string) => Promise<void>;
  verifyTotp: (code: string) => Promise<boolean>;
  logout: () => void;
}

export const useAuthStore = create<AuthStore>((set, get) => ({
  isAuthenticated: false,
  username: null,
  totpEnabled: false,

  login: async (username, _password) => {
    // TODO: Verify password with bcrypt
    set({ username, totpEnabled: true });
  },

  verifyTotp: async (code) => {
    const { username } = get();
    if (!username) return false;

    const valid = await invoke<boolean>('verify_totp', {
      username,
      code,
    });

    if (valid) {
      set({ isAuthenticated: true });
    }

    return valid;
  },

  logout: () => {
    set({ isAuthenticated: false, username: null });
  },
}));
