# Getting Started

## Installation

### Prerequisites

- Rust toolchain
- Node.js & pnpm
- WebKit2GTK (on Linux)

### Building

Clone the repository and run:

```bash
make install
```

To start the application in development mode:

```bash
make dev
```

## First Run & Authentication

Watiqa-Link uses a secure, passwordless authentication system based on **TOTP (Time-based One-Time Password)**.

### Initial Setup (Onboarding)

When you launch the application for the first time, you will see the **Secure Authentication** setup screen.

1. **Enter Username**: Provide your administrative username (default: `admin`).
2. **Scan QR Code**: A QR code will appear. Scan it using a mobile authenticator app (like Google Authenticator, Authy, or Microsoft Authenticator).
3. **Backup Codes**: You will be presented with a set of one-time emergency backup keys.
    - **Step 1**: Click **Copy Codes** (نسخ الرموز) to save them to your clipboard. Store them safely!
    - **Step 2**: Click **Complete Setup** (إتمام الإعداد) to proceed.

### Subsequent Logins

Authentication is required upon application launch.

1. Open your authenticator app.
2. Enter the 6-digit code displayed for "Watiqa-Link".
3. (Optional) If you lost your device, use one of the backup codes.

## Dashboard

Once authenticated, you will access the main dashboard where you can manage documents, view statistics, and configure system settings.
