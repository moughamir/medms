import { useState } from 'react';
import { useAuthStore } from '../stores/authStore';
import { useNavigate } from 'react-router-dom';
import { ShieldCheck, Lock, User as UserIcon, Loader2 } from 'lucide-react';

export const LoginPage = () => {
  const navigate = useNavigate();
  const { login, verifyTotp, totpEnabled, username: storedUsername } = useAuthStore();

  const [step, setStep] = useState<'login' | 'totp'>(totpEnabled ? 'totp' : 'login');
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [code, setCode] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  const handleLogin = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError('');
    try {
      await login(username, password);
      setStep('totp');
    } catch (err: any) {
      setError('خطأ في تسجيل الدخول. يرجى التأكد من البيانات.');
    } finally {
      setLoading(false);
    }
  };

  const handleVerify = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError('');
    try {
      const success = await verifyTotp(code);
      if (success) {
        navigate('/dashboard');
      } else {
        setError('رمز المصادقة غير صحيح');
      }
    } catch (err: any) {
      setError('حدث خطأ أثناء التحقق من الرمز');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-slate-50 p-4" dir="rtl">
      <div className="max-w-md w-full">
        {/* Logo/Brand Area */}
        <div className="text-center mb-8">
          <div className="inline-flex items-center justify-center w-24 h-24 bg-white rounded-full shadow-xl shadow-slate-200 mb-4 transform hover:scale-105 transition-transform border border-slate-100 p-1">
            <img src="/logo.png" alt="Morocco Administrative Police" className="w-full h-full object-contain" />
          </div>
          <h1 className="text-3xl font-bold text-slate-800 font-arabic mb-2 tracking-tight">وثيقة-لينك</h1>
          <p className="text-slate-400 font-arabic text-sm tracking-wide">المملكة المغربية - الشرطة الإدارية</p>
        </div>

        <div className="bg-white p-8 rounded-3xl shadow-xl shadow-slate-200 border border-slate-100">
          {error && (
            <div className="mb-6 p-4 bg-red-50 border-r-4 border-red-500 text-red-700 text-sm font-arabic rounded-lg animate-shake">
              {error}
            </div>
          )}

          {step === 'login' ? (
            <form onSubmit={handleLogin} className="space-y-6">
              <div className="space-y-4">
                <div className="relative">
                  <label className="text-sm font-medium text-slate-700 mb-1 block font-arabic mr-1">اسم المستخدم</label>
                  <div className="relative">
                    <input
                      type="text"
                      className="w-full pl-4 pr-11 py-3 bg-slate-50 border border-slate-200 rounded-xl focus:bg-white focus:ring-2 focus:ring-emerald-500 focus:border-transparent outline-none transition-all duration-200 text-right"
                      placeholder="اسم المستخدم"
                      value={username}
                      onChange={(e) => setUsername(e.target.value)}
                      required
                      dir="rtl"
                    />
                    <UserIcon className="absolute right-4 top-1/2 -translate-y-1/2 text-slate-400 w-5 h-5" />
                  </div>
                </div>

                <div className="relative">
                  <label className="text-sm font-medium text-slate-700 mb-1 block font-arabic mr-1">كلمة المرور</label>
                  <div className="relative">
                    <input
                      type="password"
                      className="w-full pl-4 pr-11 py-3 bg-slate-50 border border-slate-200 rounded-xl focus:bg-white focus:ring-2 focus:ring-emerald-500 focus:border-transparent outline-none transition-all duration-200 text-right"
                      placeholder="••••••••"
                      value={password}
                      onChange={(e) => setPassword(e.target.value)}
                      required
                      dir="rtl"
                    />
                    <Lock className="absolute right-4 top-1/2 -translate-y-1/2 text-slate-400 w-5 h-5" />
                  </div>
                </div>
              </div>

              <button
                type="submit"
                disabled={loading}
                className="w-full bg-emerald-600 hover:bg-emerald-700 text-white font-bold py-4 rounded-xl shadow-lg shadow-emerald-200 transition-all active:scale-[0.98] flex items-center justify-center gap-2 group font-arabic"
              >
                {loading ? <Loader2 className="w-5 h-5 animate-spin" /> : (
                  <>
                    <span>دخول</span>
                    <ShieldCheck className="w-5 h-5 group-hover:translate-x-[-2px] transition-transform" />
                  </>
                )}
              </button>
            </form>
          ) : (
            <form onSubmit={handleVerify} className="space-y-6">
              <div className="text-center mb-6">
                <div className="text-emerald-600 font-medium mb-2 font-arabic tracking-tight">مرحباً {storedUsername}</div>
                <p className="text-slate-500 text-sm font-arabic">يرجى إدخال رمز التحقق من تطبيق Authenticator</p>
              </div>

              <div className="relative">
                <input
                  type="text"
                  maxLength={6}
                  className="w-full px-4 py-4 bg-slate-50 border border-slate-200 rounded-xl focus:bg-white focus:ring-2 focus:ring-emerald-500 focus:border-transparent outline-none transition-all duration-200 text-center tracking-[1em] text-2xl font-bold font-mono"
                  placeholder="000000"
                  value={code}
                  onChange={(e) => setCode(e.target.value.replace(/[^0-9]/g, ''))}
                  autoFocus
                  required
                />
              </div>

              <button
                type="submit"
                disabled={loading || code.length !== 6}
                className="w-full bg-slate-900 hover:bg-black text-white font-bold py-4 rounded-xl shadow-lg shadow-slate-200 transition-all active:scale-[0.98] flex items-center justify-center gap-2 font-arabic disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {loading ? <Loader2 className="w-5 h-5 animate-spin" /> : "تحقق وتأكيد"}
              </button>

              <button
                type="button"
                onClick={() => setStep('login')}
                className="w-full text-slate-400 text-sm hover:text-slate-600 transition font-arabic"
              >
                الرجوع لتسجيل الدخول
              </button>
            </form>
          )}
        </div>

        <p className="text-center mt-8 text-slate-400 text-xs font-arabic">
          بوصولك لهذا النظام، فإنك توافق على سياسات الاستخدام والأمان المعمول بها.
        </p>
      </div>
    </div>
  );
};
