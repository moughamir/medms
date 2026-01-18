import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

export interface Inspection {
  id: string;
  commerce_id: string;
  inspector_id: string;
  inspection_date?: string;
  report_number: string;
  summary?: string;
  status: string;
}

export interface Violation {
  id: string;
  inspection_id: string;
  violation_code: string;
  description?: string;
  severity: string;
  measure_taken?: string;
}

export interface CreateInspection {
  commerce_id: string;
  inspector_id: string;
  report_number: string;
  summary?: string;
}

export interface CreateViolation {
  inspection_id: string;
  violation_code: string;
  description?: string;
  severity: string;
  measure_taken?: string;
}

interface InspectionState {
  inspections: Inspection[];
  violations: Violation[];
  isLoading: boolean;
  error: string | null;
  fetchInspections: (commerceId: string) => Promise<void>;
  fetchRecentInspections: () => Promise<void>;
  createInspection: (data: CreateInspection) => Promise<Inspection | null>;
  fetchViolations: (inspectionId: string) => Promise<void>;
  addViolation: (data: CreateViolation) => Promise<void>;
}

export const useInspectionStore = create<InspectionState>((set, get) => ({
  inspections: [],
  violations: [],
  isLoading: false,
  error: null,

  fetchInspections: async (commerceId) => {
    set({ isLoading: true, error: null });
    try {
      const inspections = await invoke<Inspection[]>('list_inspections_by_commerce', { commerceId });
      set({ inspections, isLoading: false });
    } catch (err) {
      set({ error: err as string, isLoading: false });
    }
  },

  fetchRecentInspections: async () => {
    set({ isLoading: true, error: null });
    try {
      const inspections = await invoke<Inspection[]>('list_recent_inspections');
      set({ inspections, isLoading: false });
    } catch (err) {
      set({ error: err as string, isLoading: false });
    }
  },

  createInspection: async (data) => {
    set({ isLoading: true, error: null });
    try {
      const inspection = await invoke<Inspection>('create_inspection', { data });
      // Update inspections list if we are in recent view or same commerce view
      // Ideally we prepend it, but depending on the view filter it might not belong.
      // For now, let's just prepend it to keep UI responsive.
      set(state => ({
        inspections: [inspection, ...state.inspections],
        isLoading: false
      }));
      return inspection;
    } catch (err) {
      set({ error: err as string, isLoading: false });
      return null;
    }
  },

  fetchViolations: async (inspectionId) => {
    try {
      const violations = await invoke<Violation[]>('list_violations', { inspectionId });
      set({ violations });
    } catch (err) {
      console.error(err);
    }
  },

  addViolation: async (data) => {
    set({ isLoading: true, error: null });
    try {
      await invoke('add_violation', { data });
      get().fetchViolations(data.inspection_id);
      set({ isLoading: false });
    } catch (err) {
      set({ error: err as string, isLoading: false });
    }
  },
}));
