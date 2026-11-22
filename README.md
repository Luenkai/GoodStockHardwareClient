# Goodstock Hardware Desktop

A native Rust / egui storefront and business workspace for Goodstock Hardware.

Author: Mark Dollentes (@Luenkai).

## Run

Install a current stable Rust toolchain and Linux desktop dependencies (including X11 and GTK development headers), then run `cargo run --release`. Windows and macOS are also supported by eframe. Run `cargo test -- --nocapture` to verify forecasting and core workflows.

The customer can browse, order, and view receipts. Delivery orders require an address. Initial administrator sign-in is `admin@goodstock.ph` / `password123`. Change this public initial password immediately using the prompted password change. Customer and administrator accounts are local to this device, and passwords are stored as salted Argon2 hashes. Existing plaintext account fields are migrated on first load; securely dispose of older backups that contain plaintext passwords. Local authentication is not protection against someone with access to the state file or application binary. Guest order history is shared on the device. This is not a multi-user server or a secure production deployment.

For GCash, set `GOODSTOCK_GCASH_QR_PAYLOAD` to the exact QR payload issued by your merchant payment provider. The counter displays that payload as a QR code; confirm its destination and test it with your provider before use. A cashier must independently confirm receipt of GCash or card payments: the app does not process, verify, or reconcile payments. Courier delivery is recorded locally; it does not dispatch a courier or calculate delivery fees.

State is stored in `goodstock_state.json` in the working directory. Back up this file; deleting it resets local inventory, accounts and orders. Set `GOODSTOCK_API_URL` to the base URL of a compatible Goodstock server and use Sync catalog to refresh product metadata and add newly discovered products asynchronously. Matching existing products retain local prices and stock. New products begin at zero stock because the endpoint does not expose inventory quantities; set stock and verify prices in Inventory before selling. Sync is read-only: orders, prices, and stock are not transmitted to the server. The forecast export writes `goodstock_models.json` in the working directory.

Catalog information is adapted from the Shopify Product Catalogue (Apache-2.0) and QTE Technologies Industrial & Scientific Product Catalog, Nguyen Huu Tuan, 2026, DOI: 10.17632/hr2ysn26ys.7 (CC BY 4.0). Prices and stock are locally maintained and should be verified before use.
