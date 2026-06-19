# Hazina Escrow - GitHub Issues List

## Frontend Issues (1-20)

### 1. Missing React Error Boundary
**Priority:** High | **Category:** Bug
The app has no error boundary to catch React rendering errors. A component crash will show a blank screen.

### 2. No Input Validation for Wallet Addresses
**Priority:** High | **Category:** Bug
SellPage accepts any string for `sellerWallet` but Stellar addresses must be exactly 56 characters starting with G.

### 3. Missing Loading State in DashboardPage
**Priority:** Medium | **Category:** Enhancement
Dashboard shows stale data while loading transactions - no skeleton or loading indicator.

### 4. No Debouncing on Search Input
**Priority:** Medium | **Category:** Performance
MarketplacePage search triggers API call on every keystroke without debouncing.

### 5. Demo Mode Hardcoded in QueryModal
**Priority:** High | **Category:** Bug
`useDemoMode` defaults to `true` - users can't actually pay with real USDC in production.

### 6. No Pagination for Marketplace Datasets
**Priority:** Medium | **Category:** Enhancement
All datasets are loaded at once - will cause performance issues with large datasets.

### 7. Missing Accessibility Attributes
**Priority:** Medium | **Category:** Accessibility
No `aria-labels` on icon-only buttons, no focus management in modals, missing alt text.

### 8. No Rate Limiting on API Calls
**Priority:** Medium | **Category:** Security
Frontend makes unlimited API requests - vulnerable to abuse.

### 9. No Confirmation for Dataset Deletion
**Priority:** Low | **Category:** UX
No confirmation dialog when users submit potentially destructive actions.

### 10. Hardcoded Particle Count in LandingPage
**Priority:** Low | **Category:** Cleanup
`Array.from({ length: 30 })` is hardcoded - should be configurable.

### 11. Missing Error Handling in useEffect
**Priority:** High | **Category:** Bug
LandingPage useEffect silently catches errors with empty catch block - users see nothing when API fails.

### 12. No TypeScript Strict Mode
**Priority:** Medium | **Category:** Cleanup
tsconfig.json missing `"strict": true` - missing null checks and type safety.

### 13. No Environment Variable Validation
**Priority:** High | **Category:** Security
App doesn't validate required env vars at startup - fails silently with cryptic errors.

### 14. Missing API Response Type Validation
**Priority:** Medium | **Category:** Bug
API responses aren't validated - malformed data causes runtime crashes.

### 15. No Request Timeout on API Calls
**Priority:** Medium | **Category:** Performance
API calls have no timeout - can hang indefinitely on slow networks.

### 16. AgentPage Missing Error Recovery
**Priority:** Medium | **Category:** Bug
When agent fails, no retry button or clear error message for user action.

### 17. No Form Auto-save
**Priority:** Low | **Category:** Enhancement
SellPage form data lost on page refresh - should persist to localStorage.

### 18. Missing Loading Skeletons
**Priority:** Low | **Category:** UX
Components show no visual feedback during initial data fetch.

### 19. No Responsive Charts on Mobile
**Priority:** Low | **Category:** UX
DashboardPage charts may overflow on small screens.

### 20. Hardcoded API Base URL
**Priority:** Medium | **Category:** Cleanup
`/api` base URL in api.ts should come from environment variable.

---

## Backend Issues (21-40)

### 21. No Input Validation/Sanitization
**Priority:** High | **Category:** Security
API endpoints accept raw user input without validation - SQL injection risk (though using JSON storage).

### 22. No Rate Limiting
**Priority:** High | **Category:** Security
Express server has no rate limiting - vulnerable to DoS attacks.

### 23. No Authentication/Authorization
**Priority:** High | **Category:** Security
All endpoints are public - anyone can create datasets or trigger payments.

### 24. Synchronous File I/O in Storage
**Priority:** Medium | **Category:** Performance
`fs.readFileSync` and `writeFileSync` block the event loop - will cause issues under load.

### 25. No Database - JSON File Storage
**Priority:** High | **Category:** Enhancement
Using JSON file (`data/datasets.json`) won't scale - needs proper database (PostgreSQL/MongoDB).

### 26. Missing Error Handling in Async Routes
**Priority:** High | **Category:** Bug
Many async route handlers don't catch errors - unhandled rejections crash the server.

### 27. No Request Logging
**Priority:** Medium | **Category:** Enhancement
No middleware for logging requests - difficult to debug and monitor.

### 28. No API Versioning
**Priority:** Medium | **Category:** Enhancement
API routes have no version prefix (`/api/v1/`) - breaking changes will affect clients.

### 29. Hardcoded Fee Percentages
**Priority:** Medium | **Category:** Cleanup
`0.95` (95%) and `0.05` (5%) fees are hardcoded in multiple places.

### 30. No Health Check for External Services
**Priority:** Medium | **Category:** Enhancement
`/health` endpoint doesn't check Stellar Horizon or Anthropic API availability.

### 31. No Transaction Atomicity
**Priority:** High | **Category:** Bug
Payment verification and data delivery aren't atomic - partial failures leave inconsistent state.

### 32. No Retry Logic for Failed Payments
**Priority:** Medium | **Category:** Bug
If seller payment fails after buyer payment succeeds, no automatic retry mechanism.

### 33. Missing Request Timeout
**Priority:** Medium | **Category:** Performance
No timeout on Stellar Horizon or Anthropic API calls - can hang indefinitely.

### 34. CORS Origin Has Hardcoded Fallback
**Priority:** Low | **Category:** Bug
`process.env.FRONTEND_URL || 'http://localhost:5173'` - dev URL in production.

### 35. No Input Validation for Wallet Addresses
**Priority:** High | **Category:** Security
Backend doesn't validate Stellar address format before storing.

### 36. No Pagination Support
**Priority:** Medium | **Category:** Enhancement
`GET /datasets` returns all datasets - needs pagination for scalability.

### 37. Missing Index on Dataset Queries
**Priority:** Medium | **Category:** Performance
No indexes on frequently queried fields (id, type) - slow lookups as data grows.

### 38. No Webhook for Payment Confirmations
**Priority:** Low | **Category:** Enhancement
No way to receive async payment confirmations - relies on polling.

### 39. Incomplete Error Responses
**Priority:** Low | **Category:** Enhancement
Error responses vary in format - should follow consistent error schema.

### 40. No Request ID for Tracing
**Priority:** Low | **Category:** Enhancement
Requests have no correlation ID - difficult to trace through logs.

---

## Smart Contract Issues (41-50)

### 41. Initialize Can Be Called Again
**Priority:** High | **Category:** Bug
`initialize()` checks if admin exists but doesn't prevent re-initialization with different params.

### 42. No Pausable Functionality
**Priority:** High | **Category:** Enhancement
No pause mechanism - can't stop contract in case of emergency.

### 43. No Upgradeability
**Priority:** High | **Category:** Enhancement
Contract is not upgradeable - bugs require full redeployment and migration.

### 44. Missing Deadline for Escrow Release
**Priority:** Medium | **Category:** Enhancement
No time limit for buyer to confirm data delivery - seller funds can be locked indefinitely.

### 45. No Buyer Confirmation Before Release
**Priority:** High | **Category:** Bug
Admin can release funds without buyer confirmation - buyer may not receive data.

### 46. Platform Fee Is Immutable
**Priority:** Medium | **Category:** Enhancement
Fee set at initialization can't be changed - can't adapt to market conditions.

### 47. No Emergency Withdrawal
**Priority:** Medium | **Category:** Enhancement
No way to recover funds if tokens get stuck (e.g., wrong token sent).

### 48. Missing Input Validation
**Priority:** High | **Category:** Bug
No validation on `amount > 0`, valid `token` address, or non-empty `dataset_id`.

### 49. No Circuit Breaker
**Priority:** Medium | **Category:** Enhancement
No mechanism to halt operations if something goes wrong (e.g., oracle failure).

### 50. No Test Coverage
**Priority:** High | **Category:** Enhancement
Contract has no unit tests - high risk of bugs in financial logic.

---

## Smart Contract — Code Correctness (51-65)

### 51. Duplicate Function Definitions Cause Compile Ambiguity
**Priority:** Critical | **Category:** Bug
`lib.rs` defines `set_default_fee`, `set_dataset_fee`, `clear_dataset_fee`, and `get_dataset_fee_config` **twice** each in the same `impl` block. Rust will compile whichever appears last, silently dropping the first definition — the first versions use correct `persistent()` storage while the duplicates use `instance()` storage, breaking per-dataset fee isolation.

### 52. `lock()` Writes Escrow Record and Increments Counter Twice
**Priority:** Critical | **Category:** Bug
In `lib.rs::lock()`, the `EscrowKey::Record(escrow_id)` is persisted and `EscrowCount` is incremented on lines ~373-382 **and again** on lines ~385-390. Every single `lock` call writes storage twice, wasting fees and making ledger history misleading.

### 53. `lock_multi()` Reads Undefined `escrow_id` Variable
**Priority:** Critical | **Category:** Bug
`lock_multi` performs a TTL bump (`extend_ttl`) and attempts to read `EscrowKey::Record(escrow_id)` before `escrow_id` is ever assigned in that scope. This orphaned block — copied from `release`/`refund` — will panic at runtime on every multi-lock call.

### 54. `refund()` Performs Duplicate State Checks
**Priority:** High | **Category:** Bug
`refund()` checks `!record.released` and `!record.refunded` with raw `assert!` macros (lines ~523-524), then immediately calls `Self::read_escrow()` and rechecks the same flags with `panic_with_error!` (lines ~525-531). The second read returns a **fresh copy** of the record, so the first `assert!` checks are against a potentially stale binding — this is a toctou-style redundancy that can mask the real error variant being propagated.

### 55. `get_escrow()` Has Unreachable Second Return
**Priority:** High | **Category:** Bug
`get_escrow` calls `.expect("escrow not found")` which already returns a value or panics, then attempts to call `Self::read_escrow` on the next line without a semicolon. The second call is unreachable dead code that will cause a compiler warning or error depending on the Rust edition.

### 56. `MAX_BASIS_POINTS` Constant Is Undefined
**Priority:** Critical | **Category:** Bug
`assert_valid_fee()` and `release_one()` reference `MAX_BASIS_POINTS` but no `const MAX_BASIS_POINTS: u32` is declared in the visible codebase. If this constant is missing the crate will fail to compile. Should be `10_000u32` (100%).

### 57. `update_fee` Uses `assert!` Instead of `panic_with_error!`
**Priority:** Medium | **Category:** Bug
`update_fee` validates the fee with `assert!(new_fee_bps <= 1_000, "fee too high")` — a plain Rust assertion — while all other validation paths use `panic_with_error!` with a typed `HazinaEscrowError`. This means callers cannot distinguish a fee-too-high failure from other panics.

### 58. Platform Fee Sent to Admin Wallet Instead of Treasury
**Priority:** High | **Category:** Security
In `release_one()`, `platform_cut` is transferred directly to the `admin` address. If admin is ever changed via `transfer_admin`, all future fees go to the new admin instantly. A dedicated immutable treasury/fee-collector address should receive platform fees.

### 59. No Minimum Lock Amount Enforced
**Priority:** Medium | **Category:** Bug
`assert_valid_amount` only checks `amount > 0` but allows `amount = 1` (1 stroop / 0.0000001 USDC). After the minimum platform fee floor of 1 stroop is applied, the seller receives 0. A sensible minimum (e.g., 10,000 stroops = 0.001 USDC) should be enforced.

### 60. `lock_multi` Doesn't Bump TTL for Newly Created Records
**Priority:** Medium | **Category:** Bug
`lock` correctly bumps TTL after writing each new `EscrowRecord`. `lock_multi` writes multiple records but never calls `extend_ttl` on any of them — the records will expire at the default Soroban TTL (~4000 ledgers ≈ 5.5 hours on mainnet).

### 61. Whitelist/Blacklist Checks Only on Buyer and Seller, Not Token
**Priority:** Medium | **Category:** Security
`require_operational_address` is called for `buyer` and `seller` in `lock()` but not for the `token` address. A blacklisted token contract could still be used as the escrow currency.

### 62. `release_multi` Doesn't Bump TTL Before Reading Records
**Priority:** Medium | **Category:** Bug
`release_one` (called by `release_multi`) calls `read_escrow` which reads without a TTL bump. If a record's TTL has expired, the read will return `EscrowNotFound` instead of the actual escrow — funds become permanently locked.

### 63. No Event Emitted on `initialize`
**Priority:** Low | **Category:** Enhancement
`initialize()` sets admin, fee, and whitelist state but emits no event. Indexers and off-chain monitors have no on-chain signal to detect contract deployment or genesis configuration.

### 64. `transfer_admin` Emits Event Without Old Admin
**Priority:** Low | **Category:** Enhancement
The `admin` event only carries `(new_admin,)` — the previous admin address is not included. Audit logs can't show who the admin was before the transfer.

### 65. No `get_escrow_count` Public Getter
**Priority:** Low | **Category:** Enhancement
`EscrowCount` is stored in instance storage but no public `get_escrow_count()` function is exposed. Indexers and frontends must scan all IDs by trial-and-error.

---

## Frontend — i18n & Localization (66-73)

### 66. WebSocket URL Uses Wrong Environment Variable Prefix
**Priority:** High | **Category:** Bug
`useTransactionWebSocket.ts` reads `process.env.REACT_APP_WEBSOCKET_URL` (line 76) — a Create React App convention. This project uses **Vite**, so all env vars must be prefixed `VITE_` and accessed via `import.meta.env`. In production the WebSocket will always fall back to `window.location.host`, breaking any separate WS server deployment.

### 67. Missing Translation Keys Silently Return Empty String
**Priority:** Medium | **Category:** Bug
`useI18n.ts` / `translate.ts` returns an empty string when a translation key is missing instead of falling back to the English key or throwing in development. Missing keys in `es.ts`, `fr.ts`, or `sw.ts` will render blank UI text with no console warning.

### 68. Swahili Translation File Missing Several Agent Keys
**Priority:** Medium | **Category:** Bug
`sw.ts` is the most recently added locale. Running a diff against `en.ts` shows `agent.scales.*` and `agent.warnings.*` keys are absent — these render as empty strings on the Agent page for Swahili users.

### 69. `LocaleSwitcher` Does Not Persist Locale to localStorage
**Priority:** Medium | **Category:** UX
Selecting a language in `LocaleSwitcher.tsx` updates in-memory state but does not write to `localStorage`. Refreshing the page resets to the browser default locale, forcing users to re-select every session.

### 70. `I18nProvider` Detects Browser Locale But Ignores Region Variants
**Priority:** Low | **Category:** Bug
`i18n/config.ts` matches `navigator.language` against supported locales (`en`, `es`, `fr`, `sw`) but `navigator.language` can return region subtags like `en-US` or `es-419`. The match fails and falls back to English even for Spanish or French speakers.

### 71. RTL Support Missing for Future Arabic/Hebrew Locales
**Priority:** Low | **Category:** Enhancement
The i18n catalog and layout do not account for RTL text direction. Adding RTL locales later will require a large layout refactor. Setting `dir` attribute on `<html>` based on locale now prevents future debt.

### 72. Number and Currency Formatting Not Localized
**Priority:** Medium | **Category:** UX
USDC amounts and query prices are formatted with hardcoded `toFixed(2)` / `toLocaleString()` calls without a locale argument. French and German users will see `.` as decimal separator instead of `,`.

### 73. Date/Time Display Not Localized
**Priority:** Low | **Category:** UX
Transaction timestamps in `DashboardPage.tsx` and `DatasetCard.tsx` use `new Date().toLocaleDateString()` without passing the current locale. Non-English users will see their OS locale format, not the app locale.

---

## Frontend — WebSocket & Real-Time (74-80)

### 74. `useTransactionWebSocket` Reconnects on Every Render Due to Unstable Callbacks
**Priority:** High | **Category:** Bug
The `connect` function lists `callbacks` in its `useCallback` dependency array (line 150). Since callers typically pass inline objects (`{ onTransactionUpdate: () => ... }`), `callbacks` gets a new reference on every render, causing infinite WebSocket disconnect/reconnect loops in `DashboardPage` and `MarketplacePage`.

### 75. WebSocket Ping Sent by Both Client and Server
**Priority:** Low | **Category:** Performance
`useTransactionWebSocket` sends a `ping` message every 25 seconds. `ws-server.ts` also runs its own heartbeat with `ping` frames from the `ws` library. This creates redundant keep-alive traffic; the server heartbeat alone is sufficient.

### 76. No WebSocket Message Size Limit
**Priority:** Medium | **Category:** Security
The WebSocket server accepts messages of unlimited size. A malicious or buggy client can send a multi-MB JSON blob that blocks the event loop while it's being parsed via `JSON.parse`. A `maxPayload` option should be set on `WebSocketServer`.

### 77. WebSocket Client Counter Never Resets
**Priority:** Low | **Category:** Bug
`clientCounter` in `WebSocketServer_Hazina` increments monotonically and never resets. After 2^53 connections (theoretical, not practical) it overflows. More practically, client IDs like `client_1000000` waste memory in the `clients` Map keys.

### 78. No Rate Limiting on WebSocket Subscribe Messages
**Priority:** Medium | **Category:** Security
A client can flood the server with repeated `subscribe` messages to grow `session.subscribed.datasetIds` without bound. No per-client subscription limit exists, enabling a memory exhaustion attack.

### 79. WebSocket Connection UI Status Not Shown to User
**Priority:** Low | **Category:** UX
`useTransactionWebSocket` exposes a `connected: boolean` and `error: string | null` state, but no page currently renders a visible connection indicator. Users have no feedback when real-time updates are unavailable.

### 80. `subscribe()` Silently Drops Messages When WebSocket Is Connecting
**Priority:** Medium | **Category:** Bug
`subscribe()` in `useTransactionWebSocket` checks `readyState === WebSocket.OPEN` and logs a warning if not. Subscriptions requested while the connection is in `CONNECTING` state are silently lost — they should be queued and sent once `onopen` fires.

---

## Frontend — Component & Page Issues (81-90)

### 81. `AgentPage` Always Calls `agentDemo` — Real Payment Path Is Unreachable
**Priority:** High | **Category:** Bug
`AgentPage.tsx::runAgent()` always calls `api.agentDemo(query)` regardless of wallet state. The code path for `api.agentResearch(query, txHash)` (real USDC payment) has no UI trigger — the page is permanently stuck in demo mode.

### 82. `AgentPage` Query Minimum Length Is Too Restrictive
**Priority:** Low | **Category:** UX
`runAgent()` returns early if `query.trim().length < 5`. A query like `"BTC?"` (4 chars) is a valid question but is silently ignored with no error message shown to the user.

### 83. `DatasetCard` `thumbnail` Falls Back to Broken `<img>` on Missing URL
**Priority:** Medium | **Category:** Bug
`DatasetCard.tsx` renders `<img src={dataset.thumbnail} />` when `thumbnail` is present. The API's `DatasetMeta` marks `thumbnail` as optional, but the component doesn't handle `404` image errors — a broken image icon is shown instead of a placeholder.

### 84. `OnboardingTour` Steps Hardcoded — No Way to Dismiss Permanently
**Priority:** Medium | **Category:** UX
`OnboardingTour.tsx` shows a guided tour on first visit but the "dismiss" state is only held in React state. Refreshing the page restarts the tour. The dismissed flag should be stored in `localStorage`.

### 85. `Navbar` Does Not Show Active Route Indicator
**Priority:** Low | **Category:** UX
`Navbar.tsx` renders navigation links without visually highlighting the current active route. Users must infer their location from page content alone — this violates basic navigation UX conventions.

### 86. `QueryModal` Does Not Clear State on Close
**Priority:** Medium | **Category:** Bug
When `QueryModal` is closed after a failed payment and reopened for the same dataset, the previous `error` and `txHash` state persists. Users see stale error messages on a fresh modal open.

### 87. `SellPage` Does Not Validate `pricePerQuery` is Positive
**Priority:** High | **Category:** Bug
The sell form sends `pricePerQuery` as a raw number input. A user can submit `0` or a negative price. The backend should reject it but currently the field is only loosely validated client-side.

### 88. `DashboardPage` Stats Section Shows `NaN` on Zero Transactions
**Priority:** Medium | **Category:** Bug
When `totalUsdcEarned` is `0` and the component computes a percentage change (e.g., `(current / previous) * 100`), it produces `NaN` or `Infinity`. These are rendered raw in the UI.

### 89. `MarketplacePage` Filter State Lost on Navigation
**Priority:** Low | **Category:** UX
Search query, type filters, price range, and sort order are kept in React state. Navigating to a dataset detail and pressing Back resets all filters. Filter state should be synced to the URL query string.

### 90. `SkeletonLoader` Hard-codes Fixed Height — Mismatches Dynamic Content
**Priority:** Low | **Category:** UX
`SkeletonLoader.tsx` uses fixed pixel heights for skeleton elements. When the real content renders with variable height (long dataset names, multi-line descriptions), the layout shift is noticeable. Skeleton dimensions should match the real content's expected dimensions.

---

## Backend — Storage & Data Layer (91-100)

### 91. JSON File Storage Has No Write-Ahead Log
**Priority:** High | **Category:** Bug
`storage.ts` serializes mutations through a promise queue but writes the entire JSON blob atomically. A process crash mid-write corrupts `datasets.json` completely. A write-ahead or temp-file-then-rename approach should be used.

### 92. `DATA_PATH` Uses `__dirname` — Broken in ESM / Docker
**Priority:** Medium | **Category:** Bug
`storage.ts` computes `DATA_PATH` via `path.join(__dirname, '../../../data/datasets.json')`. When the backend is compiled to ESM or runs inside a Docker container where the working directory differs, `__dirname` resolves incorrectly and datasets can't be loaded.

### 93. `pendingTxHashes` Set Never Cleaned Up on Error
**Priority:** Medium | **Category:** Bug
`pendingTxHashes` in `storage.ts` is populated when a payment verification starts to prevent replay. If the async path throws before the hash is removed, the hash stays in the in-memory set until process restart — blocking legitimate retry attempts.

### 94. Backup Scheduler Reads Entire Store on Every Backup
**Priority:** Medium | **Category:** Performance
`BackupScheduler` calls `readStore()` which loads the full JSON file into memory. As `datasets.json` grows, nightly backups will spike memory usage. An incremental or stream-based backup should be used.

### 95. Dataset `data` Field Stored Without Size Limit
**Priority:** High | **Category:** Security
`POST /api/datasets` accepts a `data: unknown` payload and stores it directly in the JSON file. The backend limits request body size to `2mb` (main.ts line 38) but a single very large dataset payload near that limit will bloat `datasets.json` permanently.

### 96. Transactions Array Is Never Pruned
**Priority:** Medium | **Category:** Performance
`addTransaction` appends to `store.transactions` indefinitely. With high query volume, the transactions array will grow unboundedly, making every `readStore()` call slower and every backup larger. A rolling window or archival strategy is needed.

### 97. `readStore` Called Inside Request Handlers Without Caching
**Priority:** Medium | **Category:** Performance
Every API request that needs datasets calls `readStore()` which reads from disk. Under concurrent load, many parallel disk reads compete. An in-memory cache invalidated on write would eliminate the redundant I/O.

### 98. `drizzle.config.ts` Exists but Drizzle Is Never Used
**Priority:** Medium | **Category:** Cleanup
A `drizzle.config.ts` and `drizzle/0000_initial.sql` exist, suggesting a planned migration to a real database. The file creates confusion because the actual storage layer still uses JSON. The config should be removed until the migration is complete or the migration should be completed.

### 99. `seed.ts` Overwrites Production Data if Run Accidentally
**Priority:** High | **Category:** Security
`backend/src/seed.ts` writes hardcoded demo datasets to `datasets.json` without checking for an environment guard (`NODE_ENV !== 'production'`). Running `ts-node src/seed.ts` in production wipes real user data.

### 100. Webhook Secret Stored in Plaintext in `datasets.json`
**Priority:** High | **Category:** Security
`WebhookSubscription.secret` is stored verbatim in the JSON file. If the file is accidentally exposed (backup leak, misconfigured S3 bucket), all seller webhook secrets are compromised. Secrets should be stored as BCrypt/Argon2 hashes and verified at dispatch time.

---

## Backend — Agent Service (101-108)

### 101. `AGENT_FEE_USDC` Is Hardcoded to 1 USDC
**Priority:** Medium | **Category:** Enhancement
`agent.service.ts` defines `AGENT_FEE_USDC = 1` as a constant with no environment variable override. Changing the fee requires a code deployment. It should be configurable via `process.env.AGENT_FEE_USDC`.

### 102. Agent Purchases All Seller Types Even When Only Some Are Available
**Priority:** Medium | **Category:** Bug
The research pipeline iterates all four `SELLER_TYPES` and attempts a purchase for each. If fewer than four matching datasets exist (common in early deployments), the agent still runs the report with partial data but charges the full fee — misleading the user about what they paid for.

### 103. Agent Wallet Private Key Logged on Startup
**Priority:** Critical | **Category:** Security
`agent.wallet.ts` should be reviewed: if the wallet initialization logs any key material (even first/last characters of the secret key) for debugging, those lines must be removed. Key material must never appear in logs shipped to Datadog or Sentry.

### 104. No Idempotency Key for Agent Research Jobs
**Priority:** Medium | **Category:** Bug
If a user submits the same `txHash` for `POST /api/agent/research` twice (e.g., due to a network retry), `txHashUsed` prevents the second verification but the error response looks the same as an invalid hash — there's no way to retrieve the first job result.

### 105. Research Report `rawAnalysis` Included in Every Response
**Priority:** Medium | **Category:** Performance
`AgentReport.rawAnalysis` contains the full raw Anthropic API response text and is returned to every frontend client. This field can be several kilobytes. It should be stripped before sending to the client or moved behind a separate "details" endpoint.

### 106. `synthesizeResearch` Makes Uncached Anthropic API Calls
**Priority:** Medium | **Category:** Performance
`ai/research.service.ts` calls the Anthropic API on every agent job without prompt caching. Identical or near-identical dataset summaries passed as context could benefit from Anthropic's prompt caching, reducing latency and cost.

### 107. Agent Service Has No Job Queue — Concurrent Requests Race
**Priority:** High | **Category:** Bug
Multiple simultaneous `POST /api/agent/research` calls all run the full pipeline concurrently (Stellar verification → dataset purchases → Anthropic call → storage write). Under load, parallel runs can drain the agent wallet faster than expected and produce duplicate `addTransaction` writes.

### 108. Demo Agent Route Returns Fake Payments Without Disclaimer in Body
**Priority:** Medium | **Category:** UX
`GET /api/agent/research/demo` returns a response with `payments.sellerPayments` containing realistic-looking tx hashes. The only flag is the top-level `"demo": true`. Frontend code that doesn't check this flag will display fabricated payment histories as real.

---

## Backend — Payments & Stellar (109-115)

### 109. Stellar Payment Verification Does Not Check Asset Code
**Priority:** High | **Category:** Security
`stellar.service.ts::verifyStellarPayment` verifies that the correct amount was sent to the escrow wallet but should also assert the asset code is `USDC` and the issuer is the correct Circle issuer. A payment in XLM or a fake "USDC" token from a different issuer would pass verification.

### 110. No Minimum Confirmation Depth for Stellar Transactions
**Priority:** Medium | **Category:** Security
Stellar achieves finality in one ledger close (~5 seconds), but `verifyStellarPayment` calls the Horizon API once and accepts the result immediately. A transient Horizon inconsistency or a pathological sequence could return a `not_found` before the ledger is indexed. A short retry with backoff should be used.

### 111. Escrow Wallet Balance Not Checked Before Seller Payout
**Priority:** High | **Category:** Bug
`sendUsdcPayment` in `agent.wallet.ts` attempts to send USDC to sellers without first checking that the agent wallet has sufficient balance. If the wallet is drained, the payout call throws a Stellar error and the buyer has already paid — no automatic refund path exists.

### 112. Hardcoded Stellar Horizon URL in `stellar.config.ts`
**Priority:** Medium | **Category:** Enhancement
`HORIZON_URL` should be configurable via `STELLAR_HORIZON_URL` env var to support switching between Mainnet, Testnet, and Futurenet without code changes. Currently a production → testnet switch requires editing source.

### 113. Memo Field in Payment Not Validated Against Expected Dataset ID
**Priority:** Medium | **Category:** Security
When verifying a Stellar payment, the memo field is read but not validated against the expected dataset ID. A buyer could reuse a memo-less payment hash for a different dataset if the amount matches.

### 114. No Dead-Letter Queue for Failed Seller Payments
**Priority:** High | **Category:** Bug
When a seller USDC payment fails (e.g., Stellar network error), the error is caught and logged but no retry or compensation mechanism exists. The seller never receives payment and there is no admin alert or pending-payment record.

### 115. `payments.router.ts` Exposes Raw Stellar Error Messages to Client
**Priority:** Medium | **Category:** Security
Stellar SDK errors often include internal details (sequence numbers, account details). Propagating them raw in `res.json({ error: err.message })` leaks infrastructure information. All Stellar errors should be mapped to generic user-facing messages before responding.

---

## Backend — Monitoring & Observability (116-120)

### 116. Datadog Integration Sends No Custom Metrics
**Priority:** Low | **Category:** Enhancement
`common/datadog.ts` initializes the Datadog tracer but no custom metrics (e.g., `payments.verified`, `agent.jobs.completed`, `datasets.queried`) are instrumented. Without domain-specific metrics, dashboards rely entirely on APM traces, making business monitoring difficult.

### 117. Sentry `captureException` Called Without User Context
**Priority:** Medium | **Category:** Enhancement
`Sentry.captureException(err)` in `main.ts` has no `setUser` or `setTag` context. When investigating a Sentry error, there is no way to identify which wallet or dataset was involved, making root-cause analysis slower.

### 118. Health Endpoint Returns 503 When Anthropic Key Is Missing
**Priority:** Medium | **Category:** Bug
`checkAnthropic()` returns `'error'` if `ANTHROPIC_API_KEY` env var is absent, causing the health endpoint to return `503`. In a demo-only deployment without a real key the service appears unhealthy to load balancers, which may restart or stop routing traffic.

### 119. No Structured Logging — `console.log` Used Throughout
**Priority:** Medium | **Category:** Enhancement
All logging uses `console.log` / `console.error` with template strings. This makes log-level filtering, JSON parsing in Datadog/Sentry, and log sampling impossible. A structured logger (e.g., `pino`) should be used.

### 120. Backup Router Is Publicly Accessible Without Auth
**Priority:** High | **Category:** Security
`backupRouter` is mounted at `/api` with no authentication middleware. Any unauthenticated caller can `GET /api/backup/list` to enumerate backup files, or `POST /api/backup/run` to trigger an on-demand backup and map the server filesystem layout.

---

## Infrastructure & DevOps (121-130)

### 121. `docker-compose.yml` Mounts `./data` Volume Without Explicit Permissions
**Priority:** Medium | **Category:** Security
The data directory containing `datasets.json` is bind-mounted from the host with default permissions. If the host user running Docker differs from the container user, the backend either can't write data (crash) or writes as root (security risk). Explicit user mapping should be added.

### 122. Production `docker-compose.prod.yml` Has No Resource Limits
**Priority:** Medium | **Category:** Enhancement
Neither the frontend nor backend service in `docker-compose.prod.yml` defines CPU or memory limits. An agent research job that triggers a large Anthropic response can cause the container to OOM-kill the entire service.

### 123. Terraform State File Committed or Unprotected
**Priority:** High | **Category:** Security
`terraform/` contains Terraform configuration with no `backend` block, meaning state defaults to a local `terraform.tfstate` file. If `.gitignore` doesn't exclude `*.tfstate`, secrets (API keys, credentials) embedded in state are committed to git history.

### 124. Pulumi Stack Config Not Encrypted
**Priority:** Medium | **Category:** Security
`pulumi/Pulumi.yaml` uses the default Pulumi state backend. Without explicitly setting `encryptionsalt` or using Pulumi ESC / Secrets Manager, any secret passed to the stack (`ANTHROPIC_API_KEY`, `STELLAR_SECRET_KEY`) is stored in plaintext in the state file.

### 125. No Container Image Vulnerability Scanning in CI
**Priority:** Medium | **Category:** Security
The Docker images for frontend and backend are built but not scanned for CVEs (e.g., with `trivy` or `grype`) before deployment. A known vulnerability in a base image layer could go undetected indefinitely.

### 126. Backend Dockerfile Uses `node:latest` Base Image
**Priority:** Medium | **Category:** Security
Using `latest` as the Docker base tag means the image silently upgrades on rebuild, potentially introducing breaking changes. A pinned version tag (e.g., `node:22.4-alpine`) ensures reproducible and auditable builds.

### 127. Frontend Build Artifacts Not Cache-Busted in Docker
**Priority:** Low | **Category:** Performance
The frontend Dockerfile copies source files and runs `npm run build` but doesn't leverage Docker layer caching optimally (copying `package.json` first). Every source change triggers a full `npm install`, slowing CI by minutes.

### 128. No Staging Environment Defined in Infrastructure
**Priority:** Medium | **Category:** Enhancement
All infrastructure (Terraform, Pulumi, Docker Compose) targets production only. There is no staging environment, meaning every change is tested directly in production. A parity staging environment with separate Stellar Testnet wallets should be provisioned.

### 129. `scripts/contracts` Directory Has No README
**Priority:** Low | **Category:** Documentation
The `scripts/contracts/` directory likely contains deployment and interaction scripts for the Soroban contract, but there is no README explaining what each script does, what environment variables are required, or what order to run them in.

### 130. No Dependency Update Automation
**Priority:** Low | **Category:** Enhancement
The project has no Dependabot or Renovate configuration. Security patches for `express`, `stellar-sdk`, `soroban-sdk`, and React won't be surfaced automatically.

---

## Testing & Quality (131-140)

### 131. `a11y.test.tsx` Coverage Is Incomplete
**Priority:** Medium | **Category:** Testing
`frontend/src/a11y.test.tsx` exists but only covers a subset of pages. `AgentPage`, `SellPage`, and `DashboardPage` are missing from accessibility tests, leaving ARIA violations undetected in the pages with the most interactive elements.

### 132. No Integration Tests for the Full Buy Flow
**Priority:** High | **Category:** Testing
`payments.integration.test.ts` tests Stellar payment verification in isolation. There is no end-to-end test that covers the full buyer flow: `POST /query/:id` → Stellar payment → `POST /verify/:id` → data delivery. A real Testnet integration test would catch multi-step regressions.

### 133. Smart Contract Fuzz Tests Gated Behind Feature Flag
**Priority:** Medium | **Category:** Testing
Property-based / fuzz tests in `lib.rs` are wrapped in `#[cfg(all(test, feature = "fuzz-tests"))]`. They are never run in CI because `fuzz-tests` feature is not enabled in the standard test command. These tests should be part of the default test suite.

### 134. `QueryModal.test.tsx` Does Not Test Error States
**Priority:** Medium | **Category:** Testing
`QueryModal.test.tsx` tests the happy path (payment succeeds) but has no tests for: network timeout, invalid tx hash, already-used tx hash, or server 500 errors. These are the most likely failure modes in production.

### 135. `api.test.ts` Mocks Do Not Validate Request Body Shape
**Priority:** Low | **Category:** Testing
`frontend/src/lib/api.test.ts` mocks `fetch` and asserts the correct URL is called but doesn't assert the request body (`JSON.stringify` content). A breaking change to request body shape would go undetected.

### 136. No Snapshot Tests for Critical UI Components
**Priority:** Low | **Category:** Testing
`DatasetCard.stories.tsx` and `QueryModal.stories.tsx` have Storybook stories but no Chromatic/snapshot integration. Visual regressions in the most-used components are not caught by CI.

### 137. Backend Test Coverage Has No Threshold Enforcement
**Priority:** Medium | **Category:** Testing
`vitest.config.ts` runs tests but does not set a `coverage.thresholds` object. Coverage can drop to 0% without failing CI. A minimum threshold (e.g., 70% lines) should be enforced.

### 138. `payments.integration.test.ts` Hits Live Stellar Testnet
**Priority:** Medium | **Category:** Testing
Integration tests that make real Testnet HTTP calls are flaky — they fail when Horizon is slow or the test account has no XLM. These tests should use a local mock Horizon or be tagged as `@slow` and excluded from the default test run.

### 139. No Contract Upgrade Path Tested
**Priority:** Medium | **Category:** Testing
There are no tests for the scenario where the contract address changes and existing escrow records (created under the old contract) need to be migrated or honoured. This is a critical scenario for a financial contract.

### 140. `codecov.yml` Threshold Is Set to 0%
**Priority:** Medium | **Category:** Testing
`codecov.yml` exists but sets coverage thresholds to 0% or is left at defaults, meaning Codecov never blocks a PR for coverage drops. The threshold should match the actual project baseline.

---

## Documentation & Developer Experience (141-150)

### 141. Swagger Docs Generated but Never Served
**Priority:** Medium | **Category:** Bug
`main.ts` imports `swagger-ui-express` and calls `swaggerJsdoc(swaggerOptions)` storing the result in `const _swaggerDocs` (prefixed `_` to suppress the unused warning). The Swagger UI is never mounted at any route. Developers and API consumers have no interactive documentation.

### 142. API Routes Lack JSDoc `@swagger` Annotations
**Priority:** Low | **Category:** Documentation
Even when Swagger UI is properly mounted, the `swaggerJsdoc` scan of `./src/**/*.ts` will find no `@swagger` annotations — none of the route files (`datasets.router.ts`, `payments.router.ts`, `agent.router.ts`) contain OpenAPI comments.

### 143. `CONTRIBUTING.md` Does Not Describe Smart Contract Development Setup
**Priority:** Medium | **Category:** Documentation
`CONTRIBUTING.md` exists but does not explain how to build, test, or deploy the Soroban contract. Contributors unfamiliar with `cargo`, `soroban-cli`, and the Stellar test environment have no starting point.

### 144. `README.md` Architecture Diagram Is Missing
**Priority:** Low | **Category:** Documentation
The README describes the platform at a high level but contains no architecture diagram showing how the frontend, backend, agent, smart contract, and Stellar network interact. New contributors must reverse-engineer the flow from code.

### 145. Environment Variables Not Documented with Examples
**Priority:** Medium | **Category:** Documentation
`backend/.env.example` exists but frontend environment variables (`VITE_API_URL`, `VITE_API_KEY`, `VITE_WEBSOCKET_URL`) have no corresponding `.env.example`. Contributors running the frontend locally must discover these variables by reading source code.

### 146. `backend/TODO.md` Contains Stale Tasks
**Priority:** Low | **Category:** Cleanup
`backend/TODO.md` likely contains outdated action items that have been partially addressed (e.g., "add rate limiting" — now done). Stale TODOs mislead contributors into thinking work is still needed. It should be triaged, converted to GitHub issues, and deleted.

### 147. No `SECURITY.md` or Responsible Disclosure Policy
**Priority:** Medium | **Category:** Documentation
The repository handles financial escrow logic and real USDC payments but has no `SECURITY.md` file declaring a vulnerability disclosure process. GitHub recommends this file for any project handling sensitive operations.

### 148. `contracts/hazina-escrow/README.md` Lacks Fee Calculation Examples
**Priority:** Low | **Category:** Documentation
The contract README should include worked examples showing how `platform_fee_bps` translates to actual stroop amounts for common transaction sizes (0.1 USDC, 1 USDC, 100 USDC) so integrators can verify their fee calculations.

### 149. No Changelog / Release Notes File
**Priority:** Low | **Category:** Documentation
The project has no `CHANGELOG.md`. Contributors and users cannot see what changed between versions, what bugs were fixed, or what breaking changes were introduced. A `CHANGELOG.md` following Keep a Changelog conventions should be maintained.

### 150. Storybook Configuration Outdated — Stories Don't Render in Latest Storybook 8
**Priority:** Medium | **Category:** Bug
`DatasetCard.stories.tsx` and `QueryModal.stories.tsx` use the CSF3 story format. If the Storybook version in `package.json` was upgraded to v8 without updating the `preview.ts` config and addon list, stories will fail to render, blocking visual review workflows.