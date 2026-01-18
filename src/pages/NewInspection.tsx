import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { useInspectionStore, CreateViolation } from '../stores/inspectionStore';
import { useCommerceStore } from '../stores/commerceStore';
import { useAuthStore } from '../stores/authStore';
import { ChevronRight, ChevronLeft, Save, Plus, Trash2 } from 'lucide-react';

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
          className="p-2 border rounded"
          placeholder="رمز المخالفة (Code)"
          value={code}
          onChange={e => setCode(e.target.value)}
        />
        <select
          className="p-2 border rounded"
          value={severity}
          onChange={e => setSeverity(e.target.value)}
        >
          <option value="low">منخفضة (Low)</option>
          <option value="medium">متوسطة (Medium)</option>
          <option value="high">شديدة (High)</option>
          <option value="critical">خطيرة (Critical)</option>
        </select>
        <textarea
          className="p-2 border rounded col-span-2"
          placeholder="وصف المخالفة"
          value={desc}
          onChange={e => setDesc(e.target.value)}
        />
      </div>
      <div className="flex justify-end gap-2">
        <button onClick={onCancel} className="px-3 py-1 text-slate-500">إلغاء</button>
        <button onClick={handleSubmit} className="px-3 py-1 bg-blue-600 text-white rounded">إضافة</button>
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
        <div className={`w-8 h-8 rounded-full flex items-center justify-center ${step >= 1 ? 'bg-blue-600 text-white' : 'bg-slate-200'}`}>1</div>
        <div className={`flex-1 h-1 mx-2 ${step >= 2 ? 'bg-blue-600' : 'bg-slate-200'}`} />
        <div className={`w-8 h-8 rounded-full flex items-center justify-center ${step >= 2 ? 'bg-blue-600 text-white' : 'bg-slate-200'}`}>2</div>
        <div className={`flex-1 h-1 mx-2 ${step >= 3 ? 'bg-blue-600' : 'bg-slate-200'}`} />
        <div className={`w-8 h-8 rounded-full flex items-center justify-center ${step >= 3 ? 'bg-blue-600 text-white' : 'bg-slate-200'}`}>3</div>
      </div>

      <div className="bg-white p-6 rounded-xl border border-slate-100 shadow-sm min-h-[400px]">
        {step === 1 && (
          <div className="space-y-4">
            <div className="flex justify-between items-center">
              <h2 className="text-xl font-bold font-arabic">اختيار المحل التجاري</h2>
              {selectedCommerce && (
                <button
                  onClick={() => setSelectedCommerce('')}
                  className="text-sm text-blue-600 hover:text-blue-800 font-arabic"
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
                    placeholder="ابحث عن المحل بالاسم، المسير أو الرقم السري..."
                    className="w-full p-3 pr-10 border rounded-lg bg-slate-50 focus:bg-white focus:ring-2 focus:ring-blue-500 transition-all font-arabic"
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
                        className="text-right p-4 rounded-xl border border-slate-100 hover:border-blue-200 hover:bg-blue-50/50 transition-all group flex items-start gap-4"
                      >
                        <div className="bg-slate-100 p-2 rounded-lg group-hover:bg-blue-100 group-hover:text-blue-600 transition-colors">
                          <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4" />
                          </svg>
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
              <div className="bg-blue-50 border border-blue-100 rounded-xl p-6 text-center animate-in fade-in zoom-in duration-300">
                <div className="inline-flex items-center justify-center w-16 h-16 bg-blue-100 text-blue-600 rounded-full mb-4">
                  <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M5 13l4 4L19 7" />
                  </svg>
                </div>
                <h3 className="text-xl font-bold text-blue-900 mb-1">
                  {commerces.find(c => c.id === selectedCommerce)?.name}
                </h3>
                <p className="text-blue-700 font-arabic">تم اختيار المحل بنجاح</p>
              </div>
            )}
          </div>
        )}

        {step === 2 && (
          <div className="space-y-4">
            <h2 className="text-xl font-bold font-arabic">بيانات المحضر</h2>
            <div>
              <label className="block text-sm font-medium mb-1">رقم المحضر (Report Number)</label>
              <input
                className="w-full p-3 border rounded-lg"
                value={reportNumber}
                onChange={e => setReportNumber(e.target.value)}
                placeholder="2024/001"
              />
            </div>
            <div>
              <label className="block text-sm font-medium mb-1">ملخص (Summary)</label>
              <textarea
                className="w-full p-3 border rounded-lg h-32"
                value={summary}
                onChange={e => setSummary(e.target.value)}
                placeholder="وصف ظروف المعاينة..."
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
                className="flex items-center gap-2 text-blue-600 hover:bg-blue-50 px-3 py-2 rounded transition"
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
            className="flex items-center gap-2 px-6 py-2 rounded-lg bg-blue-600 text-white hover:bg-blue-700 shadow-md shadow-blue-200"
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

