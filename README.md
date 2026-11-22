# Goodstock Hardware Desktop

A native Rust / egui storefront and business workspace for Goodstock Hardware.

Author: Mark Dollentes (@Luenkai).

## Run

Install a current stable Rust toolchain and Linux desktop dependencies (including X11 and GTK development headers), then run `cargo run --release`. Windows and macOS are also supported by eframe. Run `cargo test -- --nocapture` to verify forecasting parity.

The customer can browse, order, and view receipts. Sign in as `admin@goodstock.ph` with `password123` to access inventory, the counter and forecasting. This is an **offline, local-only interface**, not a secure authentication or payment system: customer passwords are stored in plaintext in `goodstock_state.json`, and the fixed administrator credentials are public. Do not use real passwords or deploy it for sensitive or actual transactions. GCash and card payments require external confirmation; no payment is processed. Guest order history is shared on the device.

State is stored in `goodstock_state.json` in the working directory. Back up this file; deleting it resets local inventory and orders. Set `GOODSTOCK_API_URL` to the base URL of a compatible Goodstock server and use Sync catalog to refresh matching product descriptions and categories asynchronously. The endpoint does not provide inventory quantities, so local stock and prices are retained. The forecast export writes `goodstock_models.json` in the working directory.

Catalog information is adapted from the Shopify Product Catalogue (Apache-2.0) and QTE Technologies Industrial & Scientific Product Catalog, Nguyen Huu Tuan, 2026, DOI: 10.17632/hr2ysn26ys.7 (CC BY 4.0). Prices and stock are locally maintained and should be verified before use.
