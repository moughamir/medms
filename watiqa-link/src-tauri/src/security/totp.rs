use anyhow::{Context, Result};
use base32::Alphabet;
use ring::hmac;
use serde::{Deserialize, Serialize};

const DEFAULT_DIGITS: u32 = 6;
const DEFAULT_PERIOD: u64 = 30;
const VALIDATION_WINDOW: i64 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotpSecret {
    pub secret: String,
    pub issuer: String,
    pub account: String,
    pub digits: u32,
    pub period: u64,
}

impl TotpSecret {
    pub fn generate(issuer: &str, account: &str) -> Result<Self> {
        use ring::rand::{SecureRandom, SystemRandom};

        let rng = SystemRandom::new();
        let mut bytes = vec![0u8; 20];
        rng.fill(&mut bytes)
            .map_err(|_| anyhow::anyhow!("Random gen failed"))?;

        let secret = base32::encode(Alphabet::Rfc4648 { padding: false }, &bytes);

        Ok(Self {
            secret,
            issuer: issuer.to_string(),
            account: account.to_string(),
            digits: DEFAULT_DIGITS,
            period: DEFAULT_PERIOD,
        })
    }

    pub fn provisioning_uri(&self) -> String {
        format!(
            "otpauth://totp/{}:{}?secret={}&issuer={}&digits={}&period={}",
            urlencoding::encode(&self.issuer),
            urlencoding::encode(&self.account),
            self.secret,
            urlencoding::encode(&self.issuer),
            self.digits,
            self.period
        )
    }

    pub fn generate_code(&self, time: Option<u64>) -> Result<String> {
        let timestamp = time.unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0)
        });

        let counter = timestamp / self.period;
        self.generate_hotp(counter)
    }

    pub fn verify_code(&self, code: &str, time: Option<u64>) -> Result<bool> {
        let timestamp = time.unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0)
        });

        let current_counter = (timestamp / self.period) as i64;

        for offset in -VALIDATION_WINDOW..=VALIDATION_WINDOW {
            let counter = (current_counter + offset) as u64;
            let expected_code = self.generate_hotp(counter)?;

            // Use constant-time comparison to prevent timing attacks
            if ring::constant_time::verify_slices_are_equal(
                expected_code.as_bytes(),
                code.as_bytes(),
            )
            .is_ok()
            {
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn generate_hotp(&self, counter: u64) -> Result<String> {
        let secret_bytes = base32::decode(Alphabet::Rfc4648 { padding: false }, &self.secret)
            .context("Invalid Base32 secret")?;

        let counter_bytes = counter.to_be_bytes();
        let key = hmac::Key::new(hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY, &secret_bytes);
        let tag = hmac::sign(&key, &counter_bytes);
        let hmac_result = tag.as_ref();

        let offset = (hmac_result[hmac_result.len() - 1] & 0x0F) as usize;
        let binary = u32::from_be_bytes([
            hmac_result[offset] & 0x7F,
            hmac_result[offset + 1],
            hmac_result[offset + 2],
            hmac_result[offset + 3],
        ]);

        let otp = binary % 10_u32.pow(self.digits);
        Ok(format!("{:0width$}", otp, width = self.digits as usize))
    }
}

// Helper trait to emulate into_result for easier unwrap_or_else usage if desired,
// but let's just use match or if let for clarity if possible.
// Actually, let's keep it simple.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_totp_generation_and_verification() {
        let totp = TotpSecret::generate("Test", "user@example.com").unwrap();
        let code = totp.generate_code(None).unwrap();
        assert!(totp.verify_code(&code, None).unwrap());
    }

    #[test]
    fn test_totp_window() {
        let totp = TotpSecret::generate("Test", "user@example.com").unwrap();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let code_past = totp.generate_code(Some(now - 30)).unwrap();
        let code_now = totp.generate_code(Some(now)).unwrap();
        let code_future = totp.generate_code(Some(now + 30)).unwrap();

        assert!(totp.verify_code(&code_past, Some(now)).unwrap());
        assert!(totp.verify_code(&code_now, Some(now)).unwrap());
        assert!(totp.verify_code(&code_future, Some(now)).unwrap());
    }
}
