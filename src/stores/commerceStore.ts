import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

export interface Commerce {
  id: string;
  name: string;
  address?: string;
  city?: string;
  commune?: string;
  arrondissement?: string;
  owner_name?: string;
  cin?: string;
  phone?: string;
  patente?: string;
  activity_type?: string;
  status: string;
  created_at?: string;
  updated_at?: string;
}

export interface CreateCommerce {
  name: string;
  address?: string;
  city?: string;
  commune?: string;
  arrondissement?: string;
  owner_name?: string;
  cin?: string;
  phone?: string;
  patente?: string;
  activity_type?: string;
}

interface CommerceState {
  commerces: Commerce[];
  isLoading: boolean;
  error: string | null;
  fetchCommerces: (filter?: string) => Promise<void>;
  addCommerce: (data: CreateCommerce) => Promise<void>;
  updateCommerce: (id: string, data: CreateCommerce) => Promise<void>;
  deleteCommerce: (id: string) => Promise<void>;
}

export const useCommerceStore = create<CommerceState>((set, get) => ({
  commerces: [],
  isLoading: false,
  error: null,

  fetchCommerces: async (filter) => {
    set({ isLoading: true, error: null });
    try {
      const commerces = await invoke<Commerce[]>('list_commerces', { filter });
      set({ commerces, isLoading: false });
    } catch (err) {
      set({ error: err as string, isLoading: false });
    }
  },

  addCommerce: async (data) => {
    set({ isLoading: true, error: null });
    try {
      await invoke('create_commerce', { data });
      get().fetchCommerces(); // Refresh list
    } catch (err) {
      set({ error: err as string, isLoading: false });
    }
  },

  updateCommerce: async (id, data) => {
    set({ isLoading: true, error: null });
    try {
      await invoke('update_commerce', { id, data });
      get().fetchCommerces();
    } catch (err) {
      set({ error: err as string, isLoading: false });
    }
  },

  deleteCommerce: async (id) => {
    set({ isLoading: true, error: null });
    try {
      await invoke('delete_commerce', { id });
      get().fetchCommerces();
    } catch (err) {
      set({ error: err as string, isLoading: false });
    }
  },
}));
