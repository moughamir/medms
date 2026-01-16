# Watiqa Mobile: Android Implementation

## Platform Decision: Tauri v2 Mobile

**Why Tauri Mobile over React Native/Flutter:**
- **Code Reuse**: 95% shared with desktop (Rust backend + React frontend)
- **Security**: Rust memory safety + Android KeyStore integration
- **Size**: ~8-12 MB APK vs 20-40 MB for RN/Flutter
- **Offline**: Native SQLite, no JS bridge overhead

## Architecture Additions

```
┌─────────────────────────────────────────┐
│        Android UI (React)               │
│  ┌──────────────┐  ┌──────────────┐    │
│  │ Camera       │  │ TOTP Scanner │    │
│  │ (OCR scan)   │  │ (QR reader)  │    │
│  └──────────────┘  └──────────────┘    │
└─────────────────┬───────────────────────┘
                  │
           Tauri Bridge
                  │
┌─────────────────▼───────────────────────┐
│       Rust Core (Tauri Mobile)          │
│  ┌──────────────┐  ┌──────────────┐    │
│  │ Android APIs │  │ Biometric    │    │
│  │ (JNI/FFI)    │  │ (Fingerprint)│    │
│  └──────────────┘  └──────────────┘    │
│  ┌──────────────────────────────────┐  │
│  │  TOTP Engine (ring + base32)     │  │
│  └──────────────────────────────────┘  │
└─────────────────────────────────────────┘
```

## Mobile-Specific Features

### 1. Camera Integration (OCR for Paper Mail)

**Use Case**: Scan paper documents at Bureau d'Ordre

```rust
// src-tauri/src/mobile/camera.rs
use tauri_plugin_camera::{Camera, CameraOptions};

#[tauri::command]
pub async fn scan_document() -> Result<String, String> {
    let camera = Camera::new();
    let photo = camera.take_photo(CameraOptions {
        quality: 90,
        ..Default::default()
    }).await?;
    
    // OCR with Tesseract
    let text = tesseract::ocr_from_bytes(
        &photo.data,
        "ara+fra" // Arabic + French
    )?;
    
    Ok(text)
}
```

**Cargo.toml:**
```toml
tauri-plugin-camera = "2.0"
tesseract-rs = "0.1"
```

### 2. TOTP QR Scanner

```rust
#[tauri::command]
pub async fn scan_totp_qr() -> Result<TotpSecret, String> {
    use tauri_plugin_barcode_scanner::BarcodeScanner;
    
    let scanner = BarcodeScanner::new();
    let result = scanner.scan().await?;
    
    // Parse otpauth:// URI
    parse_provisioning_uri(&result.text)
}

fn parse_provisioning_uri(uri: &str) -> Result<TotpSecret, String> {
    // Parse: otpauth://totp/Issuer:account?secret=XXX&issuer=YYY...
    let url = url::Url::parse(uri)?;
    
    let secret = url.query_pairs()
        .find(|(k, _)| k == "secret")
        .map(|(_, v)| v.to_string())
        .ok_or("Missing secret")?;
    
    Ok(TotpSecret {
        secret,
        // ... extract other params
    })
}
```

### 3. Biometric Authentication

```rust
// src-tauri/src/mobile/biometric.rs
use tauri::plugin::{Builder, TauriPlugin};
use jni::JNIEnv;

#[tauri::command]
pub async fn authenticate_biometric() -> Result<bool, String> {
    #[cfg(target_os = "android")]
    {
        use android_biometric::BiometricPrompt;
        
        let result = BiometricPrompt::builder()
            .title("تسجيل الدخول / Connexion")
            .subtitle("استخدم بصمة الإصبع / Utilisez votre empreinte")
            .negative_button("إلغاء / Annuler")
            .authenticate()
            .await?;
        
        Ok(result.success)
    }
    
    #[cfg(not(target_os = "android"))]
    Err("Biometric not supported on this platform".to_string())
}
```

**Add to Cargo.toml:**
```toml
[target.'cfg(target_os = "android")'.dependencies]
jni = "0.21"
android-biometric = "0.1"
```

### 4. Android KeyStore Integration

```rust
// Secure TOTP secret storage using Android KeyStore
use keystore::AndroidKeyStore;

pub fn store_totp_secret(username: &str, secret: &str) -> Result<()> {
    let keystore = AndroidKeyStore::new()?;
    
    let encrypted = keystore.encrypt(
        &format!("totp_{}", username),
        secret.as_bytes(),
    )?;
    
    // Store encrypted in SQLite
    Ok(())
}

pub fn retrieve_totp_secret(username: &str) -> Result<String> {
    let keystore = AndroidKeyStore::new()?;
    let encrypted = /* fetch from SQLite */;
    
    let decrypted = keystore.decrypt(
        &format!("totp_{}", username),
        &encrypted,
    )?;
    
    Ok(String::from_utf8(decrypted)?)
}
```

## Build Configuration

### Android Gradle Setup

```gradle
// android/app/build.gradle
android {
    defaultConfig {
        applicationId "ma.bouskoura.watiqa"
        minSdkVersion 24  // Android 7.0+
        targetSdkVersion 34
        versionCode 1
        versionName "0.1.0"
    }
    
    buildTypes {
        release {
            minifyEnabled true
            proguardFiles getDefaultProguardFile('proguard-android.txt')
        }
    }
}

dependencies {
    implementation 'androidx.biometric:biometric:1.2.0-alpha05'
    implementation 'com.google.mlkit:text-recognition:16.0.0' // OCR
}
```

### Permissions (AndroidManifest.xml)

```xml
<manifest>
    <uses-permission android:name="android.permission.CAMERA" />
    <uses-permission android:name="android.permission.USE_BIOMETRIC" />
    <uses-permission android:name="android.permission.WRITE_EXTERNAL_STORAGE" 
                     android:maxSdkVersion="28" />
    <uses-permission android:name="android.permission.READ_EXTERNAL_STORAGE" />
    
    <!-- For USB sync -->
    <uses-feature android:name="android.hardware.usb.host" />
    
    <application
        android:allowBackup="false"
        android:hardwareAccelerated="true"
        android:label="Watiqa"
        android:theme="@style/AppTheme">
        <!-- Activities -->
    </application>
</manifest>
```

## Mobile UI Considerations

### 1. Responsive Layout (React)

```tsx
// src/components/MobileLayout.tsx
import { isMobile } from '@tauri-apps/api/core';

export const MobileLayout = ({ children }) => {
  const mobile = isMobile();
  
  return (
    <div className={`
      ${mobile ? 'px-4 pb-20' : 'px-8'}
      rtl:text-right
    `}>
      {children}
      
      {/* Bottom Navigation (mobile only) */}
      {mobile && (
        <nav className="fixed bottom-0 inset-x-0 bg-white border-t">
          <div className="flex justify-around py-3">
            <NavButton icon="home" label="الرئيسية" />
            <NavButton icon="scan" label="مسح" />
            <NavButton icon="docs" label="وثائق" />
          </div>
        </nav>
      )}
    </div>
  );
};
```

### 2. TOTP Setup Flow (Mobile)

```tsx
// src/pages/TotpSetup.tsx
import { setupTotp } from '@/commands/auth';
import { scanTotpQr } from '@/commands/mobile';

export const TotpSetupPage = () => {
  const [step, setStep] = useState<'qr' | 'verify'>('qr');
  const [secret, setSecret] = useState('');

  const handleSetup = async () => {
    const result = await setupTotp(username);
    setSecret(result.secret);
    setStep('verify');
  };

  return (
    <div className="p-6 text-center">
      {step === 'qr' && (
        <>
          <h2 className="text-xl font-bold mb-4">
            تفعيل المصادقة الثنائية
            <br />
            Activer l'authentification 2FA
          </h2>
          
          {/* QR Code Display */}
          <div 
            dangerouslySetInnerHTML={{ __html: result.qr_code_svg }}
            className="mx-auto w-64 h-64"
          />
          
          <p className="mt-4 text-sm">
            امسح الرمز باستخدام Google Authenticator
            <br />
            Scannez avec Google Authenticator
          </p>
          
          <button 
            onClick={() => setStep('verify')}
            className="mt-6 px-6 py-3 bg-blue-600 text-white rounded"
          >
            التالي / Suivant
          </button>
        </>
      )}
      
      {step === 'verify' && (
        <TotpVerificationForm secret={secret} />
      )}
    </div>
  );
};
```

## Offline Sync for Mobile

### USB Connection

```rust
// src-tauri/src/mobile/usb_sync.rs
use tauri::plugin::TauriPlugin;

#[tauri::command]
pub async fn detect_usb_devices() -> Result<Vec<UsbDevice>, String> {
    #[cfg(target_os = "android")]
    {
        use android_usb::UsbManager;
        
        let manager = UsbManager::get()?;
        let devices = manager.device_list()?;
        
        Ok(devices.into_iter()
            .map(|d| UsbDevice {
                vendor_id: d.vendor_id,
                product_id: d.product_id,
                name: d.product_name.unwrap_or_default(),
            })
            .collect())
    }
    
    #[cfg(not(target_os = "android"))]
    Err("USB not supported".to_string())
}

#[tauri::command]
pub async fn export_to_usb(
    device_path: String,
    since: String,
) -> Result<(), String> {
    // Export sync bundle to USB drive
    let bundle = create_sync_bundle(&since).await?;
    
    let export_path = format!("{}/watiqa_sync_{}.bundle", 
        device_path, chrono::Local::now().format("%Y%m%d"));
    
    std::fs::write(export_path, serde_json::to_vec(&bundle)?)?;
    Ok(())
}
```

### Bluetooth Sync (Optional)

```rust
// For nearby device sync (e.g., tablet to desktop in same office)
use tauri_plugin_bluetooth::Bluetooth;

#[tauri::command]
pub async fn sync_via_bluetooth() -> Result<(), String> {
    let bt = Bluetooth::new()?;
    let devices = bt.discover_devices().await?;
    
    // Find Watiqa device
    let target = devices.iter()
        .find(|d| d.name.contains("Watiqa"))
        .ok_or("No Watiqa device found")?;
    
    // Transfer sync bundle
    bt.connect(target.address).await?;
    bt.send_file("sync_bundle.watiqa").await?;
    
    Ok(())
}
```

## Testing Strategy

### Android Emulator Setup

```bash
# Create AVD with API 30 (Android 11)
avdmanager create avd \
    --name watiqa_test \
    --package "system-images;android-30;google_apis;x86_64" \
    --device "pixel_4"

# Start emulator
emulator -avd watiqa_test -no-snapshot-load
```

### Integration Tests

```rust
// tests/mobile_integration.rs
#[cfg(target_os = "android")]
#[tokio::test]
async fn test_totp_generation() {
    let secret = TotpSecret::generate("Test", "user@test.com").unwrap();
    let code1 = secret.generate_code(None).unwrap();
    
    assert_eq!(code1.len(), 6);
    assert!(secret.verify_code(&code1, None).unwrap());
}

#[cfg(target_os = "android")]
#[tokio::test]
async fn test_biometric_auth() {
    // Mock biometric prompt
    let result = authenticate_biometric().await;
    assert!(result.is_ok());
}
```

## Deployment

### Build Commands

```bash
# Android APK (Debug)
pnpm tauri android dev

# Android APK (Release)
pnpm tauri android build

# Android AAB (Google Play)
pnpm tauri android build --target aab

# Desktop (unchanged)
pnpm tauri build
```

### Code Signing

```bash
# Generate keystore
keytool -genkey -v \
    -keystore watiqa-release.keystore \
    -alias watiqa \
    -keyalg RSA \
    -keysize 2048 \
    -validity 10000

# Sign APK
jarsigner -verbose \
    -sigalg SHA256withRSA \
    -digestalg SHA-256 \
    -keystore watiqa-release.keystore \
    app-release-unsigned.apk watiqa
```

## Security Checklist (Mobile)

- [ ] TOTP secrets stored in Android KeyStore (hardware-backed)
- [ ] Biometric fallback to PIN/password
- [ ] Certificate pinning for API calls (if cloud sync added)
- [ ] ProGuard/R8 obfuscation enabled
- [ ] Root detection (SafetyNet Attestation)
- [ ] Screenshot prevention in sensitive screens
- [ ] Wipe data after 10 failed auth attempts

## Updated Cost Estimate

| Item | Hours | Rate (MAD) | Total |
|------|-------|------------|-------|
| Backend (Desktop) | 160 | 500 | 80,000 |
| Frontend (React shared) | 120 | 400 | 48,000 |
| **Mobile Integration** | **80** | **500** | **40,000** |
| **TOTP + Biometric** | **40** | **500** | **20,000** |
| Testing & QA | 80 | 350 | 28,000 |
| **Total** | | | **216,000** |
| Contingency (15%) | | | 32,400 |
| **Grand Total (HT)** | | | **248,400** |
| TVA (20%) | | | 49,680 |
| **Total TTC** | | | **298,080** |

## Licensing (Updated)

**Per-Installation:**
- **Desktop License**: 50,000 MAD (unlimited users, one office)
- **Mobile Add-on**: +15,000 MAD (5 devices)
- **Additional Devices**: 2,500 MAD per device
- **Annual Support**: 12,000 MAD (desktop + mobile)

**Example Pricing:**
- Bureau d'Ordre (3 desktops + 2 tablets): 50,000 + 15,000 = **65,000 MAD**
- Services (5 additional tablets): 5 × 2,500 = **12,500 MAD**
- **Total**: 77,500 MAD + 12,000 support = **89,500 MAD/year**
