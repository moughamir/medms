import { Link, Outlet, useLocation, useNavigate } from 'react-router-dom';
import { LayoutDashboard, Store, ClipboardList, LogOut, User, Settings } from 'lucide-react';
import { useAuthStore } from '../stores/authStore';

export const DashboardLayout = () => {
  const location = useLocation();
  const navigate = useNavigate();
  const { user, logout } = useAuthStore();

  const handleLogout = () => {
    logout();
    navigate('/login');
  };

  const isActive = (path: string) => location.pathname === path;

  return (
    <div className="flex h-screen bg-slate-50" dir="rtl">
      {/* Sidebar */}
      <aside className="w-68 bg-white border-l border-slate-200 shadow-sm flex flex-col">
        <div className="p-6 border-b border-slate-100 bg-gradient-to-br from-emerald-50 to-transparent flex flex-col items-center text-center">
          <img src="/logo.png" alt="Logo" className="w-16 h-16 mb-2 drop-shadow-sm" />
          <h1 className="text-xl font-bold font-arabic text-emerald-900 tracking-tight">وثيقة-لينك</h1>
          <p className="text-[10px] text-slate-500 font-arabic mt-1 font-medium">نظام تدبير الوثائق الإدارية</p>
        </div>

        <div className="p-4 bg-slate-50/50 border-b border-slate-100 flex items-center gap-3">
          <div className="w-10 h-10 rounded-full bg-emerald-100 flex items-center justify-center text-emerald-600 border border-emerald-200">
            <User size={18} />
          </div>
          <div className="flex-1 overflow-hidden">
            <p className="text-sm font-bold text-slate-700 truncate font-arabic">{user?.username || 'المستخدم'}</p>
            <p className="text-[10px] text-slate-500 font-arabic">{user?.role === 'admin' ? 'مدير النظام' : 'مفتش'}</p>
          </div>
        </div>

        <nav className="flex-1 p-4 space-y-1">
          <Link
            to="/dashboard"
            className={`flex items-center gap-3 px-4 py-3 rounded-xl transition font-arabic ${isActive('/dashboard') ? 'bg-emerald-600 text-white shadow-lg shadow-emerald-100' : 'text-slate-600 hover:bg-slate-50'
              }`}
          >
            <LayoutDashboard size={18} />
            <span className="text-sm font-medium">لوحة القيادة</span>
          </Link>

          <Link
            to="/commerces"
            className={`flex items-center gap-3 px-4 py-3 rounded-xl transition font-arabic ${isActive('/commerces') ? 'bg-emerald-600 text-white shadow-lg shadow-emerald-100' : 'text-slate-600 hover:bg-slate-50'
              }`}
          >
            <Store size={18} />
            <span className="text-sm font-medium">المحلات التجارية</span>
          </Link>

          <Link
            to="/inspections"
            className={`flex items-center gap-3 px-4 py-3 rounded-xl transition font-arabic ${isActive('/inspections') ? 'bg-emerald-600 text-white shadow-lg shadow-emerald-100' : 'text-slate-600 hover:bg-slate-50'
              }`}
          >
            <ClipboardList size={18} />
            <span className="text-sm font-medium">المعاينات والمخالفات</span>
          </Link>

          <Link
            to="/settings"
            className={`flex items-center gap-3 px-4 py-3 rounded-xl transition font-arabic ${isActive('/settings') ? 'bg-emerald-600 text-white shadow-lg shadow-emerald-100' : 'text-slate-600 hover:bg-slate-50'
              }`}
          >
            <Settings size={18} />
            <span className="text-sm font-medium">الإعدادات</span>
          </Link>
        </nav>

        <div className="p-4 border-t border-slate-100">
          <button
            onClick={handleLogout}
            className="flex items-center gap-3 px-4 py-3 w-full rounded-xl text-red-500 hover:bg-red-50 transition font-arabic group"
          >
            <LogOut size={18} className="group-hover:-translate-x-1 transition-transform" />
            <span className="text-sm font-medium">تسجيل الخروج</span>
          </button>
        </div>
      </aside>

      {/* Main Content */}
      <main className="flex-1 overflow-auto bg-slate-50/30">
        <div className="max-w-7xl mx-auto p-8">
          <Outlet />
        </div>
      </main>
    </div>
  );
};
