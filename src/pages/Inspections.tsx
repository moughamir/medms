import { useEffect } from 'react';
import { useInspectionStore } from '../stores/inspectionStore';
import { Link } from 'react-router-dom';
import { Plus, FileText, FileSearch, ExternalLink } from 'lucide-react';

export const InspectionsPage = () => {
  const { inspections, fetchRecentInspections } = useInspectionStore();

  useEffect(() => {
    fetchRecentInspections();
  }, [fetchRecentInspections]);

  const handleOpenDoc = async (reportNumber: string, type: 'docx' | 'pdf') => {
    // Note: This logic assumes a specific naming convention in the Generated folder
    try {
      alert(`سيتم فتح ملف الـ ${type.toUpperCase()} للمحضر ${reportNumber}`);
    } catch (err) {
      console.error(err);
    }
  };

  return (
    <div className="space-y-8 animate-in fade-in duration-500">
      <div className="flex justify-between items-end">
        <div>
          <h1 className="text-3xl font-bold text-slate-800 font-arabic mb-2 tracking-tight">سجل المعاينات</h1>
          <p className="text-slate-500 font-arabic font-medium">إدارة وتحرير محاضر الشرطة الإدارية</p>
        </div>
        <Link
          to="/inspections/new"
          className="bg-emerald-600 text-white px-6 py-3 rounded-xl font-arabic font-bold hover:bg-emerald-700 flex items-center gap-2 transition-all shadow-lg shadow-emerald-100 active:scale-95"
        >
          <Plus size={20} />
          <span>إضافة معاينة جديدة</span>
        </Link>
      </div>

      <div className="bg-white rounded-3xl border border-slate-100 shadow-sm overflow-hidden">
        <div className="overflow-x-auto">
          <table className="w-full text-right border-collapse">
            <thead className="bg-slate-50/50 text-slate-500 font-arabic text-xs uppercase tracking-wider border-b border-slate-100">
              <tr>
                <th className="p-5 font-bold">رقم المحضر</th>
                <th className="p-5 font-bold">تاريخ المعاينة</th>
                <th className="p-5 font-bold">الحالة</th>
                <th className="p-5 font-bold">ملخص الملاحظات</th>
                <th className="p-5 font-bold text-center">الإجراءات</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-slate-50">
              {(!inspections || inspections.length === 0) ? (
                <tr>
                  <td colSpan={5} className="p-12 text-center text-slate-400 font-arabic">
                    <div className="flex flex-col items-center gap-2">
                      <FileSearch size={40} className="text-slate-200" />
                      <span>لا توجد معاينات مسجلة حاليا</span>
                    </div>
                  </td>
                </tr>
              ) : (
                inspections.map((insp) => (
                  <tr key={insp.id} className="group hover:bg-emerald-50/30 transition-colors">
                    <td className="p-5">
                      <div className="flex items-center gap-3">
                        <div className="w-10 h-10 bg-slate-100 rounded-xl flex items-center justify-center text-slate-500 group-hover:bg-emerald-100 group-hover:text-emerald-600 transition-colors">
                          <FileText size={20} />
                        </div>
                        <span className="font-bold text-slate-700 font-mono text-lg">{insp.report_number}</span>
                      </div>
                    </td>
                    <td className="p-5 text-slate-600 font-medium whitespace-nowrap" dir="ltr">
                      {new Date(insp.inspection_date || '').toLocaleDateString('fr-FR')}
                    </td>
                    <td className="p-5">
                      <span className={`px-3 py-1 rounded-full text-xs font-bold font-arabic ${insp.status === 'draft' ? 'bg-amber-100 text-amber-700' :
                        insp.status === 'submitted' ? 'bg-emerald-100 text-emerald-700' :
                          'bg-slate-100 text-slate-700'
                        }`}>
                        {insp.status === 'draft' ? 'مسودة' : insp.status === 'submitted' ? 'محرر' : 'مؤرشف'}
                      </span>
                    </td>
                    <td className="p-5 text-slate-500 text-sm max-w-[200px] truncate font-arabic">{insp.summary || '-'}</td>
                    <td className="p-5">
                      <div className="flex items-center justify-center gap-2">
                        <button
                          className="p-2 text-slate-400 hover:text-emerald-600 hover:bg-emerald-50 rounded-lg transition-all"
                          title="فتح في Word"
                          onClick={() => handleOpenDoc(insp.report_number, 'docx')}
                        >
                          <ExternalLink size={18} />
                        </button>
                      </div>
                    </td>
                  </tr>
                ))
              )}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
};
