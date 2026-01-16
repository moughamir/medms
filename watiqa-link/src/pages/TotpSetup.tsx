import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { QRCodeSVG } from 'qrcode.react';

export const TotpSetupPage = () => {
  const [step, setStep] = useState<'qr' | 'verify'>('qr');
  const [qrUri, setQrUri] = useState('');
  const [backupCodes, setBackupCodes] = useState<string[]>([]);
  const [username, setUsername] = useState('admin');

  const handleSetup = async () => {
    try {
      const result = await invoke<{
        secret: string;
        qr_uri: string;
        backup_codes: string[];
      }>('setup_totp', { username });

      setQrUri(result.qr_uri);
      setBackupCodes(result.backup_codes);
      setStep('qr'); // Actually instruction says setStep('verify') but normally we show QR first
      // But user's code had setStep('verify') after handleSetup.
      // Wait, let's follow the user's logic but ensure qrUri is set.
      setStep('verify');
    } catch (e) {
      console.error(e);
    }
  };

  return (
    <div className="min-h-screen flex flex-col items-center justify-center bg-gray-100 p-4" dir="rtl">
      <div className="bg-white p-8 rounded-lg shadow-md max-w-md w-full">
        {qrUri === '' ? (
          <div className="flex flex-col gap-4">
            <input
              type="text"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              className="w-full border rounded p-2 text-center"
              placeholder="اسم المستخدم"
            />
            <button
              onClick={handleSetup}
              className="w-full bg-blue-600 text-white py-2 rounded-md hover:bg-blue-700 transition font-arabic"
            >
              بدء إعداد المصادقة الثنائية
            </button>
          </div>
        ) : (
          <>
            {step === 'qr' && (
              <div className="text-center">
                <h2 className="text-2xl font-bold mb-4 font-arabic">
                  تفعيل المصادقة الثنائية
                </h2>
                <div className="bg-white p-4 inline-block rounded-lg shadow-inner mb-4">
                  <QRCodeSVG value={qrUri} size={256} className="mx-auto" />
                </div>
                <p className="text-sm text-gray-600 mb-4 font-arabic">
                  امسح الرمز باستخدام Google Authenticator أو تطبيق مماثل
                </p>
                <button
                  onClick={() => setStep('verify')}
                  className="text-blue-600 hover:text-blue-800 font-semibold"
                >
                  التالي: رموز الاحتياط
                </button>
              </div>
            )}

            {step === 'verify' && (
              <div>
                <h3 className="text-lg font-semibold mb-4 text-center font-arabic">رموز الاحتياط</h3>
                <p className="text-xs text-red-600 mb-4 text-center font-arabic">
                  احفظ هذه الرموز في مكان آمن. ستستخدمها في حال فقدان هاتفك.
                </p>
                <div className="grid grid-cols-2 gap-2 mb-6">
                  {backupCodes.map((code, i) => (
                    <div key={i} className="font-mono bg-gray-100 p-2 rounded text-center text-sm border border-gray-200">
                      {code}
                    </div>
                  ))}
                </div>
                <button
                  onClick={() => setStep('qr')}
                  className="w-full text-gray-500 text-sm"
                >
                  العودة للرمز
                </button>
              </div>
            )}
          </>
        )}
      </div>
    </div>
  );
};
