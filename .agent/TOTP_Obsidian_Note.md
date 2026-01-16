---
title: "Time-based One-Time Password (TOTP)"
date: 2025-11-20
tags: [TOTP, authentication, security, 2FA, otp, cryptography]
categories: [Security, Authentication, Reference]
aliases: [Time-based OTP, RFC 6238]
---

# Time-based One-Time Password (TOTP)

**Topics:** Definition · Algorithm · History · Pseudocode · Implementations · References

---

## Summary
Time-based One-Time Password (TOTP) is a widely-used algorithm for generating short-lived numeric codes (one-time passwords) from a shared secret and the current time. It is an extension of HOTP (HMAC-based One-Time Password, RFC 4226) that replaces the event counter with the current time as the moving factor. TOTP is standardized by **RFC 6238**.

---

## Quick facts
- Typical time step: $T_X = 30\ \text{seconds}$  
- Typical digits: $d = 6$ (i.e., codes in range $[0, 10^6-1]$)  
- Common hash: SHA-1 (but SHA-256 and SHA-512 are permitted)  
- Secret encoding: commonly **Base32** when shown to users (e.g., provisioning URIs)

---

## Notation / Parameters
- $K$ — shared secret (binary). Often presented to users encoded in Base32.  
- $T$ — current Unix time in seconds (UTC).  
- $T_0$ — Unix epoch start for TOTP (default $T_0 = 0$).  
- $T_X$ — time step (default $T_X = 30$ seconds).  
- $C_T$ — time counter computed from $T$ and $T_0$.  
- $d$ — number of digits in OTP (commonly $6$).

The time counter is:
$$
C_T \;=\; \left\lfloor \dfrac{T - T_0}{T_X} \right\rfloor
$$

The TOTP value is derived by running the HOTP function over the time counter:
$$
\mathrm{TOTP}_d(K,T) \;=\; \mathrm{HOTP}_d\big(K,\,C_T\big)
$$
where HOTP uses an HMAC over the 8-byte big-endian representation of $C_T$, followed by *dynamic truncation*, then reduction modulo $10^d$:
$$
\mathrm{TOTP}_d = \mathrm{Truncate}(\mathrm{HMAC}(K, \mathrm{BE\_8}(C_T))) \bmod 10^d
$$

---

## How it works (step-by-step)
1. Compute $C_T$ using the current Unix time $T$ and agreed $T_X$ and $T_0$.  
2. Convert $C_T$ to an 8-byte big-endian byte array.  
3. Compute an HMAC with key $K$ over that 8-byte counter. Typical HMAC algorithm: HMAC-SHA1 (default), HMAC-SHA256, or HMAC-SHA512.  
4. Perform **dynamic truncation**: use the low 4 bits of the last hash byte to select a 4-byte (32-bit) slice from the HMAC result, clear the sign bit (most significant bit) to obtain a 31-bit positive integer.  
5. Compute `otp = binary_value mod 10^d`. Zero-pad to `d` digits for display.  
6. Validation on server side: accept codes within a small window (e.g., ±1 time step) to allow for clock skew and transmission delay.

---

## Dynamic truncation (formula)
Let `H` be the HMAC result (byte array) and `H[len(H)-1]` its last byte. Compute:
$$
\text{offset} = H[\text{len}(H)-1] \mathbin{\&} 0x0F
$$
Then build 4 bytes starting at `offset`:
$$
\text{binary} = \big( (H[\text{offset}] \mathbin{\&} 0x7F) \ll 24 \big)
+ \big( (H[\text{offset}+1] \mathbin{\&} 0xFF) \ll 16 \big)
+ \big( (H[\text{offset}+2] \mathbin{\&} 0xFF) \ll 8 \big)
+ \big( (H[\text{offset}+3] \mathbin{\&} 0xFF) \big)
$$
Finally:
$$
\mathrm{OTP} = \text{binary} \bmod 10^d
$$

---

## Pseudocode (readable, language-agnostic)
```pseudocode
# Generate TOTP (human-friendly pseudocode)
function generateTOTP(secretKey, time = currentUnixTime(), digits = 6, timestep = 30, T0 = 0, algorithm = "SHA1"):
    // 1) compute time counter
    counter = floor((time - T0) / timestep)

    // 2) convert counter to 8-byte big-endian array
    C = toBigEndianBytes(counter, length = 8)

    // 3) compute HMAC with chosen hash algorithm
    H = HMAC(hash=algorithm, key=secretKey, message=C)   // returns byte array

    // 4) dynamic truncation
    offset = H[last_index] & 0x0F
    P = ( (H[offset]   & 0x7F) << 24 )
      | ( (H[offset+1] & 0xFF) << 16 )
      | ( (H[offset+2] & 0xFF) << 8  )
      | ( (H[offset+3] & 0xFF) )

    // 5) compute OTP
    otp = P mod (10 ^ digits)

    // 6) return zero-padded string of length `digits`
    return zeroPad(otp, digits)
```

---

## Pseudocode (LaTeX algorithm environment)
The following LaTeX block works well with Obsidian's LaTeX Suite if you render math environments:

```latex
\[
\begin{algorithmic}[1]
\Function{GenerateTOTP}{$K, T, T_0, T_X, d$}
  \State $C_T \gets \left\lfloor \dfrac{T - T_0}{T_X} \right\rfloor$
  \State $C \gets \text{BE\_8}(C_T)$ \Comment{8-byte big-endian}
  \State $H \gets \mathrm{HMAC}(K, C)$
  \State $\text{offset} \gets H[\text{len}(H)-1] \mathbin{\&} 0x0F$
  \State $\text{binary} \gets ((H[\text{offset}] \mathbin{\&} 0x7F) \ll 24)$
  \State \quad $+ ((H[\text{offset}+1] \mathbin{\&} 0xFF) \ll 16)$
  \State \quad $+ ((H[\text{offset}+2] \mathbin{\&} 0xFF) \ll 8)$
  \State \quad $+ (H[\text{offset}+3] \mathbin{\&} 0xFF)$
  \State $\mathrm{OTP} \gets \text{binary} \bmod 10^d$
  \State \Return \text{ZeroPad}(\mathrm{OTP}, d)
\EndFunction
\end{algorithmic}
\]
```

> Tip: In Obsidian, ensure your LaTeX Suite is configured to render `algorithmic` or use a simpler `equation` block if packages are not available.

---

## Implementation notes & best practices
- **Clock synchronization:** The server and client clocks must be reasonably close. Allow a verification window (e.g., ±1 or ±2 time steps) to account for skew.  
- **Secret storage:** Store the shared secret $K$ securely (use HSM or encrypted storage if possible). Never transmit secrets in cleartext over insecure channels.  
- **Provisioning URI:** For QR codes and provisioning into authenticators, use the `otpauth://` URI format (Google Authenticator compatible). Example:
  ```
  otpauth://totp/Issuer:alice@example.com?secret=BASE32SECRET&algorithm=SHA1&digits=6&period=30&issuer=Issuer
  ```
- **Hash algorithms:** SHA1 is widely supported; for higher security, SHA256 or SHA512 can be used but must be agreed by both ends.  
- **Base32 decoding:** Many implementations accept a Base32 encoded secret; decode to raw bytes before HMAC.  
- **Rate limiting:** Treat OTP verification attempts as authentication attempts — apply rate limiting and lockout policies.  
- **Recovery & backup:** Provide secure backup/escape paths (e.g., recovery codes) in case the user loses the authenticator device.

---

## Popular libraries (language-agnostic list)
- **Python:** `pyotp` — https://github.com/pyauth/pyotp (`pip install pyotp`)  
- **Node.js / JavaScript:** `otplib`, `speakeasy` — npm packages `otplib`, `speakeasy`  
- **Go:** `github.com/pquerna/otp` — `go get github.com/pquerna/otp`  
- **Java:** `jchambers/java-otp` (GitHub)  
- **C# / .NET:** `Otp.NET` (NuGet package `Otp.NET`)  
- **Ruby:** `rotp` (gem `rotp`)  
- **PHP:** `PHPGangsta/GoogleAuthenticator`, other RFC-conformant libs  
- **Rust:** `rust-otp` / crates implementing TOTP/HOTP

> Search the language's package registry for "TOTP" or "HOTP" for more options. Most libraries expose easy functions to generate and validate tokens, handle Base32 secrets, and adjust parameters.

---

## History & Origins (concise)
- HOTP (RFC 4226) introduced HMAC-based one-time passwords using a counter as the moving factor.  
- OATH (Initiative for Open Authentication) produced drafts and reference specs for a time-based variant — TOTP — that uses time as the moving factor.  
- TOTP was published as **RFC 6238** (May 2011) and quickly became the de-facto standard for many consumer 2FA apps (e.g., Google Authenticator, Authy).  
- The main design goal was interoperability and simplicity: shared secret + time yields short-lived codes without persistent counters.

---

## References
- Wikipedia — *Time-based one-time password* — https://en.wikipedia.org/wiki/Time-based_one-time_password  
- RFC 6238 — *TOTP: Time-Based One-Time Password Algorithm* — https://datatracker.ietf.org/doc/html/rfc6238  
- RFC 4226 — *HOTP: An HMAC-Based One-Time Password Algorithm* — https://datatracker.ietf.org/doc/html/rfc4226  
- PyOTP — https://github.com/pyauth/pyotp  
- otplib — https://github.com/yeojz/otplib  
- pquerna/otp (Go) — https://github.com/pquerna/otp

---

## Example validation window (server-side)
- Accept codes where:
  $$
  C_T - W \le C_\text{candidate} \le C_T + W
  $$
  where $W$ is the allowed window (e.g., $W=1$ allows ±30s if $T_X=30$). Adjust based on your UX/security tradeoffs.

---

## Obsidian-specific tips
- Because you have the **LaTeX Suite** plugin, the algorithmic LaTeX block should render if the required LaTeX packages are available. If the package `algorithmic` or `algorithm` is not available in your renderer, stick to the pseudocode fenced block.  
- Use `[[TOTP Implementation Checklist]]` to create a task page linking to deployment steps (provisioning, storage, rotation, rotation policy).  
- Add callouts using Obsidian's syntax:
  > [!warning] Secrets must be stored securely — never hard-code secrets in source.

---

## TODO / Checklist for implementers
- [ ] Choose hash algorithm (SHA1 / SHA256 / SHA512)  
- [ ] Choose timestep and digits  
- [ ] Implement Base32 secret decoding  
- [ ] Implement TOTP generation + validation with a window  
- [ ] Securely store secret keys  
- [ ] Add provisioning QR generator (otpauth://)  
- [ ] Add rate-limiting and monitoring for verification attempts

---

*Last updated: 2025-11-20*
