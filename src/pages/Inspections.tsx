import { useEffect } from 'react';
import { useInspectionStore } from '../stores/inspectionStore';
import { invoke } from '@tauri-apps/api/core';

export const InspectionsPage = () => {
  const { inspections, isLoading } = useInspectionStore((state) => ({
    ...state,
    // We might need to add a separate state for recent inspections or just reuse inspections
    // For now abusing inspections array. 
  }));

  // Quick hack: Update store to support fetching recent
  const fetchRecent = async () => {
    try {
      const data = await invoke<any[]>('list_recent_inspections');
      useInspectionStore.setState({ inspections: data });
    } catch (e) {
      console.error(e);
    }
  };

  useEffect(() => {
    fetchRecent();
  }, []);

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-2xl font-bold text-slate-900 font-arabic">المعاينات والمخالفات</h1>
          <p className="text-slate-500 font-arabic">سجل عمليات المراقبة وتحرير المحاضر</p>
        </div>
        <a href="/inspections/new" className="bg-blue-600 text-white px-4 py-2 rounded-lg font-arabic hover:bg-blue-700 flex items-center gap-2">
          <span>+ معاينة جديدة</span>
        </a>
      </div>

      {isLoading && <div className="text-center py-10">جاري التحميل...</div>}

      <div className="bg-white rounded-xl border border-slate-100 shadow-sm overflow-hidden">
        <table className="w-full text-right">
          <thead className="bg-slate-50 text-slate-600 font-arabic text-sm">
            <tr>
              <th className="p-4 font-medium">رقم المحضر</th>
              <th className="p-4 font-medium">تاريخ المعاينة</th>
              <th className="p-4 font-medium">الحالة</th>
              <th className="p-4 font-medium">ملخص</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-slate-100">
            {(!inspections || inspections.length === 0) ? (
              <tr>
                <td colSpan={4} className="p-8 text-center text-slate-500 font-arabic">
                  لا توجد معاينات مسجلة مؤخرا
                </td>
              </tr>
            ) : (
              inspections.map((insp) => (
                <tr key={insp.id} className="hover:bg-slate-50 transition">
                  <td className="p-4 font-bold text-slate-800">{insp.report_number}</td>
                  <td className="p-4 text-slate-600" dir="ltr">{insp.inspection_date?.replace('T', ' ')}</td>
                  <td className="p-4">
                    <span className={`px-2 py-1 rounded-full text-xs font-medium ${insp.status === 'draft' ? 'bg-yellow-100 text-yellow-700' :
                      insp.status === 'submitted' ? 'bg-blue-100 text-blue-700' :
                        'bg-green-100 text-green-700'
                      }`}>
                      {insp.status === 'draft' ? 'مسودة' : 'محال'}
                    </span>
                  </td>
                  <td className="p-4 text-slate-500 text-sm max-w-xs truncate">{insp.summary || '-'}</td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
};
