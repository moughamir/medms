import { useEffect, useState } from 'react';
import { useCommerceStore } from '../stores/commerceStore';
import { Plus, Search, MapPin, Phone, FileText } from 'lucide-react';

export const CommercesPage = () => {
  const { commerces, isLoading, fetchCommerces, addCommerce } = useCommerceStore();
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [searchTerm, setSearchTerm] = useState('');

  // Form State
  const [formData, setFormData] = useState({
    name: '',
    owner_name: '',
    activity_type: '',
    city: 'Casablanca',
    commune: 'Bouskoura'
  });

  useEffect(() => {
    fetchCommerces();
  }, [fetchCommerces]);

  const handleSearch = (e: React.ChangeEvent<HTMLInputElement>) => {
    setSearchTerm(e.target.value);
    // Debounce ideally, but direct call for now
    fetchCommerces(e.target.value);
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    await addCommerce(formData);
    setIsModalOpen(false);
    setFormData({ name: '', owner_name: '', activity_type: '', city: 'Casablanca', commune: 'Bouskoura' });
  };

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-2xl font-bold text-slate-900 font-arabic">سجل المحلات التجارية</h1>
          <p className="text-slate-500 font-arabic">إدارة وتتبع المحلات والأنشطة الاقتصادية</p>
        </div>
        <button
          onClick={() => setIsModalOpen(true)}
          className="bg-emerald-600 hover:bg-emerald-700 text-white px-4 py-2.5 rounded-lg flex items-center gap-2 font-arabic shadow-sm transition"
        >
          <Plus size={18} />
          <span>إضافة محل جديد</span>
        </button>
      </div>

      {/* Search Bar */}
      <div className="relative">
        <input
          type="text"
          placeholder="بحث باسم المحل، المالك، أو رقم البطاقة الوطنية..."
          value={searchTerm}
          onChange={handleSearch}
          className="w-full pl-4 pr-10 py-3 rounded-xl border border-slate-200 focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 outline-none transition font-arabic"
        />
        <Search className="absolute right-3 top-3.5 text-slate-400" size={20} />
      </div>

      {/* Loading State */}
      {isLoading && <div className="text-center py-10">جاري التحميل...</div>}

      {/* Grid List */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {commerces.map((commerce) => (
          <div key={commerce.id} className="bg-white p-5 rounded-xl border border-slate-100 hover:shadow-md transition group">
            <div className="flex justify-between items-start mb-3">
              <div className="w-10 h-10 rounded-lg bg-emerald-100 text-emerald-600 flex items-center justify-center font-bold text-lg">
                {commerce.name.charAt(0).toUpperCase()}
              </div>
              <span className={`px-2 py-1 rounded text-xs font-medium ${commerce.status === 'active' ? 'bg-green-100 text-green-700' : 'bg-red-100 text-red-700'}`}>
                {commerce.status === 'active' ? 'نشط' : 'مغلق'}
              </span>
            </div>

            <h3 className="font-bold text-lg text-slate-800 mb-1">{commerce.name}</h3>
            <p className="text-slate-500 text-sm mb-4">{commerce.activity_type || 'نشاط غير محدد'}</p>

            <div className="space-y-2 text-sm text-slate-600">
              <div className="flex items-center gap-2">
                <FileText size={16} className="text-slate-400" />
                <span>{commerce.owner_name || 'غير معروف'}</span>
              </div>
              <div className="flex items-center gap-2">
                <MapPin size={16} className="text-slate-400" />
                <span className="truncate">{commerce.address || commerce.commune}</span>
              </div>
              {commerce.phone && (
                <div className="flex items-center gap-2">
                  <Phone size={16} className="text-slate-400" />
                  <span className="font-mono text-xs" dir="ltr">{commerce.phone}</span>
                </div>
              )}
            </div>
          </div>
        ))}
      </div>

      {/* Add Modal */}
      {isModalOpen && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
          <div className="bg-white rounded-2xl w-full max-w-lg overflow-hidden shadow-2xl animate-fade-in-up">
            <div className="p-6 border-b border-gray-100 flex justify-between items-center">
              <h3 className="text-xl font-bold font-arabic">تسجيل محل جديد</h3>
              <button
                onClick={() => setIsModalOpen(false)}
                className="text-gray-400 hover:text-gray-600"
              >
                ✕
              </button>
            </div>

            <form onSubmit={handleSubmit} className="p-6 space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1 font-arabic">اسم المحل (Enseigne)</label>
                <input
                  type="text"
                  required
                  value={formData.name}
                  onChange={e => setFormData({ ...formData, name: e.target.value })}
                  className="w-full px-4 py-2 rounded-lg border border-gray-300 focus:ring-2 focus:ring-emerald-500 outline-none"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1 font-arabic">اسم المالك</label>
                <input
                  type="text"
                  value={formData.owner_name}
                  onChange={e => setFormData({ ...formData, owner_name: e.target.value })}
                  className="w-full px-4 py-2 rounded-lg border border-gray-300 focus:ring-2 focus:ring-emerald-500 outline-none"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1 font-arabic">نوع النشاط</label>
                <input
                  type="text"
                  value={formData.activity_type}
                  onChange={e => setFormData({ ...formData, activity_type: e.target.value })}
                  className="w-full px-4 py-2 rounded-lg border border-gray-300 focus:ring-2 focus:ring-emerald-500 outline-none"
                />
              </div>

              <div className="flex justify-end gap-3 mt-6">
                <button
                  type="button"
                  onClick={() => setIsModalOpen(false)}
                  className="px-4 py-2 text-gray-600 hover:bg-gray-50 rounded-lg font-arabic"
                >
                  إلغاء
                </button>
                <button
                  type="submit"
                  className="px-6 py-2 bg-emerald-600 hover:bg-emerald-700 text-white rounded-lg font-bold font-arabic"
                >
                  حفظ البيانات
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
};
