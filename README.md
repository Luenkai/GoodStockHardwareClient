# Goodstock Hardware Desktop

A native Rust / egui storefront and business workspace for Goodstock Hardware.

Author: Mark Dollentes (@Luenkai).

## Download a desktop app

Open **Actions → Desktop downloads**, select a successful run, and download the artifact for your computer (GitHub sign-in is required; artifacts expire after 90 days):

- **Windows x64:** extract the downloaded zip and double-click `Goodstock Hardware.exe`. Windows may warn that this application is unsigned.
- **macOS:** choose Apple silicon or Intel, extract the artifact and then the included macOS zip, and open `Goodstock Hardware.app`. This build is ad-hoc signed, not notarized; macOS may block it. Use Finder's Open action and follow macOS security prompts if you trust the download.
- **Linux x64:** extract the artifact and the included `.tar.gz`, then launch `Goodstock Hardware` from your file manager. The binary requires a compatible Linux desktop with standard GTK 3 / X11 or Wayland libraries. Some file managers require enabling the executable permission first.

These are test builds, not signed installers. Local data is stored separately from the extracted application, so updating the app does not normally erase it. No terminal is needed for Windows or macOS launch.

## Build from source

Install a current stable Rust toolchain and Linux desktop dependencies (including X11 and GTK development headers), then run `cargo run --release`. Windows and macOS are also supported by eframe. Run `cargo test -- --nocapture` to verify forecasting and core workflows.

The customer can browse, order, and view receipts. Delivery orders require an address. Initial administrator sign-in is `admin@goodstock.ph` / `password123`. Change this public initial password immediately using the prompted password change. Customer and administrator accounts are local to this device, and passwords are stored as salted Argon2 hashes. Existing plaintext account fields are migrated on first load; securely dispose of older backups that contain plaintext passwords. Local authentication is not protection against someone with access to the state file or application binary. Guest order history is shared on the device. This is not a multi-user server or a secure production deployment.

For GCash, set `GOODSTOCK_GCASH_QR_PAYLOAD` to the exact QR payload issued by your merchant payment provider. The counter displays that payload as a QR code; confirm its destination and test it with your provider before use. A cashier must independently confirm receipt of GCash or card payments: the app does not process, verify, or reconcile payments. Courier delivery is recorded locally; it does not dispatch a courier or calculate delivery fees.

State is stored in `goodstock_state.json` in the per-user application data directory (shown in administrator Settings). Older state files in the working directory are loaded if no per-user file exists; future saves use the per-user directory. Back up this file; deleting it resets local inventory, accounts and orders. Enter the HTTPS base URL of a compatible Goodstock server in Settings, or set `GOODSTOCK_API_URL` on desktop and use Sync catalog to refresh product metadata and add newly discovered products asynchronously. Matching existing products retain local prices and stock. New products begin at zero stock because the endpoint does not expose inventory quantities; set stock and verify prices in Inventory before selling. Sync is read-only: orders, prices, and stock are not transmitted to the server. The forecast export writes `goodstock_models.json` in the same data directory.

Catalog information is adapted from the Shopify Product Catalogue (Apache-2.0) and QTE Technologies Industrial & Scientific Product Catalog, Nguyen Huu Tuan, 2026, DOI: 10.17632/hr2ysn26ys.7 (CC BY 4.0). Prices and stock are locally maintained and should be verified before use.

## Android APK

The Android build supports 64-bit ARM devices running Android 6.0 or newer. Open the repository's **Actions → Android APK** workflow, select a successful run and download the **goodstock-hardware-android-arm64** artifact. Unzip it on your phone and open the `.apk`; Android may ask you to allow installation from the app you used to download it. GitHub artifact downloads require signing in and expire after 90 days. This is a debug-signed build for testing, not a Play Store release. Debug keys are generated per build, so uninstalling a previous build may be necessary before installing a newer one; uninstalling removes local app data.

The Android app stores data in its private app storage. It is separate from desktop state and is removed on uninstall. Use the in-app Settings after signing in as administrator to set the HTTPS API base URL and merchant QR payload; Android apps cannot use desktop shell environment variables. The app uses an on-screen keyboard for text entry. Desktop builds are provided separately as described above.
