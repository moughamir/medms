import { useSettingsStore } from '../stores/settingsStore';
import { Save, Building2, MapPin, Shield, FileSpreadsheet } from 'lucide-react';
import { invoke } from '@tauri-apps/api/core';

export const SettingsPage = () => {
  const { organization, updateOrganization } = useSettingsStore();

  const handleSave = () => {
    // Already synced via updateOrganization calls or we can add a notification here
    alert('تم حفظ الإعدادات بنجاح');
  };

  const handleExportLedger = async () => {
    try {
      await invoke('export_ledger');
      alert('تم تحديث سجل Excel بنجاح في مجلد Documents/WatiqaLink');
    } catch (err) {
      alert('فشل تصدير السجل: ' + err);
    }
  };

  return (
    <div className="max-w-4xl mx-auto space-y-8 animate-in fade-in slide-in-from-bottom-4 duration-500">
      <div className="flex justify-between items-center">
        <h1 className="text-3xl font-bold text-slate-800 font-arabic">الإعدادات العامة</h1>
        <div className="flex gap-4">
          <button
            onClick={handleExportLedger}
            className="flex items-center gap-2 bg-white text-emerald-600 border border-emerald-200 px-6 py-3 rounded-xl hover:bg-emerald-50 transition-all font-arabic font-bold shadow-sm"
          >
            <FileSpreadsheet size={20} />
            <span>تحديث سجل Excel</span>
          </button>
          <button
            onClick={handleSave}
            className="flex items-center gap-2 bg-emerald-600 text-white px-6 py-3 rounded-xl hover:bg-emerald-700 transition-all shadow-lg shadow-emerald-100 font-arabic font-bold"
          >
            <Save size={20} />
            <span>حفظ التغييرات</span>
          </button>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* Organization Card */}
        <div className="bg-white p-6 rounded-3xl border border-slate-100 shadow-sm space-y-6">
          <div className="flex items-center gap-3 text-emerald-600 border-b border-slate-50 pb-4">
            <Building2 size={24} />
            <h2 className="text-xl font-bold font-arabic text-slate-800">الهوية الإدارية</h2>
          </div>

          <div className="space-y-4">
            <div>
              <label className="block text-sm font-bold text-slate-700 mb-2 font-arabic">الدولة</label>
              <input
                className="w-full p-3 bg-slate-50 border border-slate-200 rounded-xl focus:ring-4 focus:ring-emerald-500/10 focus:border-emerald-500 outline-none transition-all font-arabic"
                value={organization.country}
                onChange={(e) => updateOrganization({ country: e.target.value })}
              />
            </div>
            <div>
              <label className="block text-sm font-bold text-slate-700 mb-2 font-arabic">الوزارة الوصية</label>
              <input
                className="w-full p-3 bg-slate-50 border border-slate-200 rounded-xl focus:ring-4 focus:ring-emerald-500/10 focus:border-emerald-500 outline-none transition-all font-arabic"
                value={organization.ministry}
                onChange={(e) => updateOrganization({ ministry: e.target.value })}
              />
            </div>
          </div>
        </div>

        {/* Location Card */}
        <div className="bg-white p-6 rounded-3xl border border-slate-100 shadow-sm space-y-6">
          <div className="flex items-center gap-3 text-emerald-600 border-b border-slate-50 pb-4">
            <MapPin size={24} />
            <h2 className="text-xl font-bold font-arabic text-slate-800">المجال الترابي</h2>
          </div>

          <div className="space-y-4">
            <div>
              <label className="block text-sm font-bold text-slate-700 mb-2 font-arabic">العمالة / الإقليم</label>
              <input
                className="w-full p-3 bg-slate-50 border border-slate-200 rounded-xl focus:ring-4 focus:ring-emerald-500/10 focus:border-emerald-500 outline-none transition-all font-arabic"
                value={organization.province}
                onChange={(e) => updateOrganization({ province: e.target.value })}
              />
            </div>
            <div>
              <label className="block text-sm font-bold text-slate-700 mb-2 font-arabic">الجماعة</label>
              <input
                className="w-full p-3 bg-slate-50 border border-slate-200 rounded-xl focus:ring-4 focus:ring-emerald-500/10 focus:border-emerald-500 outline-none transition-all font-arabic"
                value={organization.commune}
                onChange={(e) => updateOrganization({ commune: e.target.value })}
              />
            </div>
          </div>
        </div>

        {/* Department Card */}
        <div className="bg-white p-6 rounded-3xl border border-slate-100 shadow-sm space-y-6 col-span-1 md:col-span-2">
          <div className="flex items-center gap-3 text-emerald-600 border-b border-slate-50 pb-4">
            <Shield size={24} />
            <h2 className="text-xl font-bold font-arabic text-slate-800">المصلحة المختصة</h2>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <label className="block text-sm font-bold text-slate-700 mb-2 font-arabic">القسم / المصلحة</label>
              <input
                className="w-full p-3 bg-slate-50 border border-slate-200 rounded-xl focus:ring-4 focus:ring-emerald-500/10 focus:border-emerald-500 outline-none transition-all font-arabic"
                value={organization.department}
                onChange={(e) => updateOrganization({ department: e.target.value })}
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
