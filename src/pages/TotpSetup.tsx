import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { QRCodeSVG } from 'qrcode.react';

export const TotpSetupPage = () => {
  const [step, setStep] = useState<'init' | 'qr' | 'verify'>('init');
  const [qrUri, setQrUri] = useState('');
  const [backupCodes, setBackupCodes] = useState<string[]>([]);
  const [username, setUsername] = useState('admin');
  const [isLoading, setIsLoading] = useState(false);

  const handleSetup = async () => {
    setIsLoading(true);
    try {
      const result = await invoke<{
        secret: string;
        qr_uri: string;
        backup_codes: string[];
      }>('setup_totp', { username });

      setQrUri(result.qr_uri);
      setBackupCodes(result.backup_codes);
      setStep('qr');
    } catch (e) {
      console.error(e);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-slate-50 p-4" dir="rtl">
      <div className="max-w-md w-full bg-white rounded-2xl shadow-xl overflow-hidden border border-slate-100">

        {/* Header */}
        <div className="bg-emerald-600 p-6 text-center text-white">
          <h1 className="text-2xl font-bold font-arabic mb-2">مصادقة آمنة</h1>
          <p className="text-emerald-100 text-sm font-arabic">Watiqa-Link Security</p>
        </div>

        <div className="p-8">
          {step === 'init' && (
            <div className="space-y-6">
              <div className="text-center">
                <p className="text-gray-600 font-arabic mb-6">
                  للبدء، يرجى تأكيد اسم المستخدم لإنشاء مفاتيح الأمان الخاصة بك.
                </p>
              </div>

              <div className="space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1 font-arabic">اسم المستخدم</label>
                  <input
                    type="text"
                    value={username}
                    onChange={(e) => setUsername(e.target.value)}
                    className="w-full px-4 py-3 rounded-lg border border-gray-300 focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 outline-none transition text-left"
                    dir="ltr"
                  />
                </div>

                <button
                  onClick={handleSetup}
                  disabled={isLoading}
                  className="w-full bg-emerald-600 hover:bg-emerald-700 text-white font-bold py-3 rounded-lg shadow-lg hover:shadow-xl transition transform active:scale-95 flex justify-center items-center gap-2 font-arabic"
                >
                  {isLoading ? (
                    <span className="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin" />
                  ) : (
                    "إعداد المصادقة الآن"
                  )}
                </button>
              </div>
            </div>
          )}

          {step === 'qr' && (
            <div className="text-center animate-fade-in-up">
              <h2 className="text-xl font-bold text-gray-800 mb-6 font-arabic border-b pb-4">
                تفعيل المصادقة الثنائية
              </h2>

              <div className="bg-white p-4 border-2 border-dashed border-gray-300 rounded-xl inline-block mb-6">
                <QRCodeSVG value={qrUri} size={220} className="mx-auto" />
              </div>

              <div className="bg-blue-50 text-blue-800 p-4 rounded-lg text-sm mb-8 text-right">
                <ol className="list-decimal list-inside space-y-2 font-arabic">
                  <li>افتح تطبيق <strong>Google Authenticator</strong></li>
                  <li>اضغط على علامة (+) واختر "مسح رمز QR"</li>
                  <li>وجه الكاميرا نحو الرمز أعلاه</li>
                </ol>
              </div>

              <button
                onClick={() => setStep('verify')}
                className="w-full bg-gray-900 text-white font-bold py-3 rounded-lg hover:bg-gray-800 transition font-arabic"
              >
                لقد قمت بالمسح، التالي
              </button>
            </div>
          )}

          {step === 'verify' && (
            <div className="animate-fade-in-up">
              <div className="text-center mb-6">
                <div className="inline-flex items-center justify-center w-12 h-12 rounded-full bg-red-100 text-red-600 mb-4">
                  <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
                  </svg>
                </div>
                <h3 className="text-xl font-bold text-gray-900 font-arabic">رموز الطوارئ</h3>
                <p className="text-sm text-gray-500 mt-2 font-arabic">
                  يرجى حفظ هذه الرموز في مكان آمن. لن تظهر مرة أخرى.
                </p>
              </div>

              <div className="grid grid-cols-2 gap-3 mb-8 bg-gray-50 p-4 rounded-xl border border-gray-200">
                {backupCodes.map((code, i) => (
                  <div key={i} className="font-mono text-center text-sm font-semibold text-gray-700 tracking-wider">
                    {code}
                  </div>
                ))}
              </div>

              <div className="flex flex-col gap-3">
                <button
                  onClick={async () => {
                    try {
                      await navigator.clipboard.writeText(backupCodes.join('\n'));
                      alert('تم نسخ الرموز بنجاح');
                    } catch (err) {
                      console.error('Failed to copy text: ', err);
                    }
                  }}
                  className="w-full border-2 border-gray-200 text-gray-700 font-semibold py-3 rounded-lg hover:bg-gray-50 transition font-arabic"
                >
                  نسخ الرموز
                </button>

                <button
                  onClick={() => alert('Setup Complete! Navigate to Dashboard...')} // TODO: Implement navigation
                  className="w-full bg-emerald-600 hover:bg-emerald-700 text-white font-bold py-3 rounded-lg shadow-lg hover:shadow-xl transition transform active:scale-95 flex justify-center items-center gap-2 font-arabic"
                >
                  إتمام الإعداد
                </button>

                <button
                  onClick={() => setStep('qr')}
                  className="text-gray-400 text-xs hover:text-gray-600 font-arabic pt-2"
                >
                  العودة للخطوة السابقة
                </button>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
