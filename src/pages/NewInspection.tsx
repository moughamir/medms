import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { useInspectionStore } from '../stores/inspectionStore';
import { useCommerceStore } from '../stores/commerceStore';
import { ChevronRight, ChevronLeft, Save, Plus, Trash2 } from 'lucide-react';
import { CreateViolation } from '../stores/inspectionStore';

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
  const { createInspection, addViolation } = useInspectionStore();
  const { commerces, fetchCommerces } = useCommerceStore();

  const [step, setStep] = useState(1);
  const [selectedCommerce, setSelectedCommerce] = useState('');
  const [reportNumber, setReportNumber] = useState('');
  const [summary, setSummary] = useState('');
  const [violations, setViolations] = useState<CreateViolation[]>([]);
  const [showViolationForm, setShowViolationForm] = useState(false);

  useEffect(() => {
    fetchCommerces();
  }, []);

  const handleNext = () => {
    if (step === 1 && !selectedCommerce) return;
    if (step === 2 && !reportNumber) return;
    setStep(s => s + 1);
  };

  const handleBack = () => setStep(s => s - 1);

  const handleSubmit = async () => {
    // 1. Create Inspection
    // Fake inspector ID for now
    const inspection = await createInspection({
      commerce_id: selectedCommerce,
      inspector_id: '123e4567-e89b-12d3-a456-426614174000', // To be replaced by auth
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
            <h2 className="text-xl font-bold font-arabic">اختيار المحل التجاري</h2>
            <select
              className="w-full p-3 border rounded-lg bg-slate-50"
              value={selectedCommerce}
              onChange={e => setSelectedCommerce(e.target.value)}
            >
              <option value="">اختر محلاً...</option>
              {commerces.map(c => (
                <option key={c.id} value={c.id}>{c.name} - {c.owner_name}</option>
              ))}
            </select>
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
            className="flex items-center gap-2 px-6 py-2 rounded-lg bg-green-600 text-white hover:bg-green-700 shadow-md shadow-green-200"
          >
            <Save size={20} />
            <span>حفظ المحضر</span>
          </button>
        )}
      </div>
    </div>
  );
};
