import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { useInspectionStore, CreateViolation } from '../stores/inspectionStore';
import { useCommerceStore } from '../stores/commerceStore';
import { useAuthStore } from '../stores/authStore';
import { ChevronRight, ChevronLeft, Save, Plus, Trash2, Store } from 'lucide-react';

const ViolationForm = ({ onAdd, onCancel }: { onAdd: (v: CreateViolation) => void, onCancel: () => void }) => {
  const [code, setCode] = useState('');
  const [desc, setDesc] = useState('');
  const [severity, setSeverity] = useState('medium');

  const handleSubmit = () => {
    if (!code) return;
    onAdd({
      inspection_id: '', // Will be filled later
      violation_code: code,
      description: desc,
      severity,
      measure_taken: 'warning' // Default
    });
    setCode('');
    setDesc('');
  };

  return (
    <div className="bg-slate-50 p-4 rounded-lg border border-slate-200 space-y-3">
      <h4 className="font-bold text-slate-700 font-arabic">إضافة مخالفة</h4>
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <input
          className="p-3 border rounded-lg focus:ring-2 focus:ring-emerald-500 outline-none transition-all font-arabic"
          placeholder="رمز المخالفة"
          value={code}
          onChange={e => setCode(e.target.value)}
        />
        <select
          className="p-3 border rounded-lg focus:ring-2 focus:ring-emerald-500 outline-none transition-all font-arabic"
          value={severity}
          onChange={e => setSeverity(e.target.value)}
        >
          <option value="low">منخفضة</option>
          <option value="medium">متوسطة</option>
          <option value="high">شديدة</option>
          <option value="critical">خطيرة جدا</option>
        </select>
        <textarea
          className="p-3 border rounded-lg col-span-2 focus:ring-2 focus:ring-emerald-500 outline-none transition-all font-arabic h-24"
          placeholder="وصف المخالفة بالتفصيل..."
          value={desc}
          onChange={e => setDesc(e.target.value)}
        />
      </div>
      <div className="flex justify-end gap-2">
        <button onClick={onCancel} className="px-4 py-2 text-slate-500 hover:bg-slate-100 rounded-lg transition font-arabic">إلغاء</button>
        <button onClick={handleSubmit} className="px-4 py-2 bg-emerald-600 text-white rounded-lg hover:bg-emerald-700 transition font-arabic shadow-lg shadow-emerald-100">إضافة</button>
      </div>
    </div>
  );
};

export const NewInspectionPage = () => {
  const navigate = useNavigate();
  const { createInspection, addViolation, isLoading } = useInspectionStore();
  const { commerces, fetchCommerces } = useCommerceStore();
  const { user } = useAuthStore();

  const [step, setStep] = useState(1);
  const [selectedCommerce, setSelectedCommerce] = useState('');
  const [reportNumber, setReportNumber] = useState('');
  const [summary, setSummary] = useState('');
  const [violations, setViolations] = useState<CreateViolation[]>([]);
  const [showViolationForm, setShowViolationForm] = useState(false);

  useEffect(() => {
    fetchCommerces();
    // Pre-fill report number with YYYYMMDD/random format
    const now = new Date();
    const dateStr = now.getFullYear().toString() +
      (now.getMonth() + 1).toString().padStart(2, '0') +
      now.getDate().toString().padStart(2, '0');
    const randomSuffix = Math.floor(Math.random() * 1000).toString().padStart(3, '0');
    setReportNumber(`${dateStr}/${randomSuffix}`);
  }, [fetchCommerces]);

  const handleNext = () => {
    if (step === 1 && !selectedCommerce) return;
    if (step === 2 && !reportNumber) return;
    setStep(s => s + 1);
  };

  const handleBack = () => setStep(s => s - 1);

  const handleSubmit = async () => {
    // 1. Create Inspection
    // ID 123e4567-e89b-12d3-a456-426614174000 is a placeholder until Auth is fully linked
    const inspection = await createInspection({
      commerce_id: selectedCommerce,
      inspector_id: user?.id || '00000000-0000-0000-0000-000000000000',
      report_number: reportNumber,
      summary
    });

    if (inspection) {
      // 2. Add violations
      for (const v of violations) {
        await addViolation({ ...v, inspection_id: inspection.id });
      }
      navigate('/inspections');
    }
  };

  return (
    <div className="max-w-3xl mx-auto space-y-6">
      <h1 className="text-2xl font-bold font-arabic mb-6">محضر معاينة جديد</h1>

      {/* Stepper */}
      <div className="flex items-center justify-between mb-8 px-10">
        <div className={`w-10 h-10 rounded-full flex items-center justify-center font-bold transition-all ${step >= 1 ? 'bg-emerald-600 text-white shadow-xl shadow-emerald-100' : 'bg-slate-200 text-slate-500'}`}>1</div>
        <div className={`flex-1 h-1 mx-2 rounded-full transition-all ${step >= 2 ? 'bg-emerald-600' : 'bg-slate-200'}`} />
        <div className={`w-10 h-10 rounded-full flex items-center justify-center font-bold transition-all ${step >= 2 ? 'bg-emerald-600 text-white shadow-xl shadow-emerald-100' : 'bg-slate-200 text-slate-500'}`}>2</div>
        <div className={`flex-1 h-1 mx-2 rounded-full transition-all ${step >= 3 ? 'bg-emerald-600' : 'bg-slate-200'}`} />
        <div className={`w-10 h-10 rounded-full flex items-center justify-center font-bold transition-all ${step >= 3 ? 'bg-emerald-600 text-white shadow-xl shadow-emerald-100' : 'bg-slate-200 text-slate-500'}`}>3</div>
      </div>

      <div className="bg-white p-6 rounded-xl border border-slate-100 shadow-sm min-h-[400px]">
        {step === 1 && (
          <div className="space-y-4">
            <div className="flex justify-between items-center">
              <h2 className="text-xl font-bold font-arabic">اختيار المحل التجاري</h2>
              {selectedCommerce && (
                <button
                  onClick={() => setSelectedCommerce('')}
                  className="text-sm text-emerald-600 hover:text-emerald-800 font-arabic font-bold"
                >
                  تغيير الاختيار
                </button>
              )}
            </div>

            {!selectedCommerce ? (
              <div className="space-y-4">
                <div className="relative">
                  <input
                    type="text"
                    placeholder="ابحث عن المحل باسم المحل، اسم المسير أو رقم المحل..."
                    className="w-full py-4 pr-12 border border-slate-200 rounded-2xl bg-slate-50 focus:bg-white focus:ring-4 focus:ring-emerald-500/10 focus:border-emerald-500 outline-none transition-all font-arabic text-right shadow-sm"
                    onChange={(e) => fetchCommerces(e.target.value)}
                  />
                  <div className="absolute inset-y-0 right-3 flex items-center pointer-events-none text-slate-400">
                    <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                    </svg>
                  </div>
                </div>

                <div className="grid grid-cols-1 gap-2 max-h-[300px] overflow-y-auto pr-2 custom-scrollbar">
                  {commerces.length > 0 ? (
                    commerces.map((c) => (
                      <button
                        key={c.id}
                        onClick={() => setSelectedCommerce(c.id)}
                        className="text-right p-5 rounded-2xl border border-slate-100 hover:border-emerald-200 hover:bg-emerald-50/50 transition-all group flex items-start gap-4 shadow-sm hover:shadow-md"
                      >
                        <div className="bg-slate-100 p-3 rounded-xl group-hover:bg-emerald-100 group-hover:text-emerald-600 transition-colors">
                          <Store size={24} />
                        </div>
                        <div className="flex-1">
                          <h3 className="font-bold text-slate-800 mb-1">{c.name}</h3>
                          <div className="flex flex-wrap gap-x-4 gap-y-1 text-sm text-slate-500">
                            <span className="flex items-center gap-1">
                              <span className="font-medium text-slate-700 font-arabic">المسير:</span> {c.owner_name}
                            </span>
                            <span className="flex items-center gap-1">
                              <span className="font-medium text-slate-700 font-arabic">البطاقة الوطنية:</span> {c.cin}
                            </span>
                            <span className="flex items-center gap-1">
                              <span className="font-medium text-slate-700 font-arabic">رقم الضريبة:</span> {c.patente}
                            </span>
                          </div>
                        </div>
                      </button>
                    ))
                  ) : (
                    <div className="text-center py-10 text-slate-400 font-arabic">
                      لا يوجد محل بهذا الاسم
                    </div>
                  )}
                </div>
              </div>
            ) : (
              <div className="bg-emerald-50 border border-emerald-100 rounded-2xl p-8 text-center animate-in fade-in zoom-in duration-300">
                <div className="inline-flex items-center justify-center w-20 h-20 bg-emerald-100 text-emerald-600 rounded-full mb-4 shadow-inner">
                  <svg className="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="3" d="M5 13l4 4L19 7" />
                  </svg>
                </div>
                <h3 className="text-2xl font-bold text-emerald-900 mb-2 font-arabic">
                  {commerces.find(c => c.id === selectedCommerce)?.name}
                </h3>
                <p className="text-emerald-700 font-arabic font-medium">تم اختيار المحل بنجاح</p>
              </div>
            )}
          </div>
        )}

        {step === 2 && (
          <div className="space-y-4">
            <h2 className="text-xl font-bold font-arabic">بيانات المحضر</h2>
            <div>
              <label className="block text-sm font-bold text-slate-700 mb-2 font-arabic">رقم المحضر</label>
              <input
                className="w-full p-4 border border-slate-200 rounded-xl focus:ring-4 focus:ring-emerald-500/10 focus:border-emerald-500 transition-all font-mono text-center text-lg font-bold bg-slate-50"
                value={reportNumber}
                onChange={e => setReportNumber(e.target.value)}
                placeholder="20240119/001"
              />
            </div>
            <div>
              <label className="block text-sm font-bold text-slate-700 mb-2 font-arabic">ملخص الملاحظات</label>
              <textarea
                className="w-full p-4 border border-slate-200 rounded-xl h-40 focus:ring-4 focus:ring-emerald-500/10 focus:border-emerald-500 transition-all font-arabic"
                value={summary}
                onChange={e => setSummary(e.target.value)}
                placeholder="صف ظروف المعاينة والملاحظات المسجلة..."
              />
            </div>
          </div>
        )}

        {step === 3 && (
          <div className="space-y-4">
            <div className="flex justify-between items-center">
              <h2 className="text-xl font-bold font-arabic">المخالفات ({violations.length})</h2>
              <button
                onClick={() => setShowViolationForm(true)}
                className="flex items-center gap-2 text-emerald-600 hover:bg-emerald-50 px-4 py-2 rounded-xl transition font-arabic font-bold border border-emerald-100 shadow-sm"
              >
                <Plus size={18} />
                <span>إضافة مخالفة</span>
              </button>
            </div>

            {showViolationForm && (
              <ViolationForm
                onAdd={v => { setViolations([...violations, v]); setShowViolationForm(false); }}
                onCancel={() => setShowViolationForm(false)}
              />
            )}

            <div className="space-y-2">
              {violations.map((v, i) => (
                <div key={i} className="flex justify-between p-3 bg-slate-50 rounded border border-slate-100">
                  <div>
                    <span className="font-bold">{v.violation_code}</span>
                    <p className="text-sm text-slate-500">{v.description}</p>
                  </div>
                  <button onClick={() => setViolations(violations.filter((_, idx) => idx !== i))} className="text-red-500">
                    <Trash2 size={18} />
                  </button>
                </div>
              ))}
              {violations.length === 0 && !showViolationForm && (
                <p className="text-center text-slate-400 py-8">لا توجد مخالفات مسجلة</p>
              )}
            </div>
          </div>
        )}
      </div>

      <div className="flex justify-between pt-4">
        <button
          onClick={handleBack}
          disabled={step === 1}
          className="flex items-center gap-2 px-6 py-2 rounded-lg border border-slate-200 text-slate-600 disabled:opacity-50 hover:bg-slate-50"
        >
          <ChevronRight size={20} />
          <span>السابق</span>
        </button>

        {step < 3 ? (
          <button
            onClick={handleNext}
            className="flex items-center gap-2 px-8 py-3 rounded-xl bg-emerald-600 text-white hover:bg-emerald-700 shadow-lg shadow-emerald-100 transition-all active:scale-95 font-arabic font-bold"
          >
            <span>التالي</span>
            <ChevronLeft size={20} />
          </button>
        ) : (
          <button
            onClick={handleSubmit}
            disabled={isLoading}
            className="flex items-center gap-2 px-6 py-2 rounded-lg bg-green-600 text-white hover:bg-green-700 shadow-md shadow-green-200 disabled:opacity-70"
          >
            {isLoading ? <span>جاري الحفظ...</span> : (
              <>
                <Save size={20} />
                <span>حفظ المحضر</span>
              </>
            )}
          </button>
        )}
      </div>
    </div>
  );
};

