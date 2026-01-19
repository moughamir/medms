import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface SettingsState {
  organization: {
    country: string;
    ministry: string;
    province: string;
    commune: string;
    department: string;
  };
  defaults: {
    reportPrefix: string;
    city: string;
  };
  updateOrganization: (org: Partial<SettingsState['organization']>) => void;
}

export const useSettingsStore = create<SettingsState>()(
  persist(
    (set) => ({
      organization: {
        country: 'المملكة المغربية',
        ministry: 'وزارة الداخلية',
        province: 'إقليم النواصر',
        commune: 'جماعة بوسكورة',
        department: 'قسم الشرطة الإدارية',
      },
      defaults: {
        reportPrefix: 'PV',
        city: 'بوسكورة',
      },
      updateOrganization: (org) =>
        set((state) => ({
          organization: { ...state.organization, ...org },
        })),
    }),
    {
      name: 'watiqa-settings',
    }
  )
);
