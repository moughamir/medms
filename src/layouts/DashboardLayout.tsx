import { Link, Outlet, useLocation } from 'react-router-dom';
import { LayoutDashboard, Store, ClipboardList, LogOut } from 'lucide-react';

export const DashboardLayout = () => {
  const location = useLocation();

  const isActive = (path: string) => location.pathname === path;

  return (
    <div className="flex h-screen bg-slate-50" dir="rtl">
      {/* Sidebar */}
      <aside className="w-64 bg-white border-l border-slate-200 shadow-sm flex flex-col">
        <div className="p-6 border-b border-slate-100">
          <h1 className="text-xl font-bold font-arabic text-emerald-700">وثيقة-لينك</h1>
          <p className="text-xs text-slate-500 font-arabic mt-1">نظام تدبير الوثائق الإدارية</p>
        </div>

        <nav className="flex-1 p-4 space-y-2">
          <Link
            to="/dashboard"
            className={`flex items-center gap-3 px-4 py-3 rounded-lg transition font-arabic ${isActive('/dashboard') ? 'bg-emerald-50 text-emerald-700' : 'text-slate-600 hover:bg-slate-50'
              }`}
          >
            <LayoutDashboard size={20} />
            <span>لوحة القيادة</span>
          </Link>

          <Link
            to="/commerces"
            className={`flex items-center gap-3 px-4 py-3 rounded-lg transition font-arabic ${isActive('/commerces') ? 'bg-emerald-50 text-emerald-700' : 'text-slate-600 hover:bg-slate-50'
              }`}
          >
            <Store size={20} />
            <span>المحلات التجارية</span>
          </Link>

          <Link
            to="/inspections"
            className={`flex items-center gap-3 px-4 py-3 rounded-lg transition font-arabic ${isActive('/inspections') ? 'bg-emerald-50 text-emerald-700' : 'text-slate-600 hover:bg-slate-50'
              }`}
          >
            <ClipboardList size={20} />
            <span>المعاينات والمخالفات</span>
          </Link>
        </nav>

        <div className="p-4 border-t border-slate-100">
          <button className="flex items-center gap-3 px-4 py-3 w-full rounded-lg text-red-600 hover:bg-red-50 transition font-arabic">
            <LogOut size={20} />
            <span>تسجيل الخروج</span>
          </button>
        </div>
      </aside>

      {/* Main Content */}
      <main className="flex-1 overflow-auto p-8">
        <Outlet />
      </main>
    </div>
  );
};
