# Design Document: Frontend Testing Infrastructure

## Overview

The frontend already has Vitest configured (`frontend/vitest.config.ts`) with `happy-dom` as the test environment, `@testing-library/react` installed, and several existing test files. The goal is to fill the coverage gap for `AgentPage.tsx` and formalize property-based testing for pure utility functions (`localizeScale`, `formatUSDC`, `truncateAddress`).

The design follows the patterns already established in the codebase:
- Components are wrapped in `<I18nProvider initialLocale="en">` for rendering
- `vi.mock('../../lib/api', ...)` is used to stub API calls
- `fireEvent` + `waitFor` from RTL handle async interactions
- `@testing-library/react` `render`/`screen` for DOM assertions

No new dependencies are required. The existing `vitest` + `@testing-library/react` + `happy-dom` stack is sufficient. For property-based testing, we will use **fast-check** — a mature PBT library with first-class TypeScript support that integrates cleanly with Vitest.

## Architecture

```
frontend/
├── vitest.config.ts          (existing — no changes needed)
├── src/
│   ├── pages/
│   │   ├── AgentPage.tsx     (existing component)
│   │   └── AgentPage.test.tsx  ← NEW
│   ├── components/ui/
│   │   └── QueryModal.test.tsx (existing — already covers happy path & error)
│   └── lib/
│       ├── utils.ts          (existing pure functions)
│       └── utils.test.ts       ← NEW (property tests)
```

The `AgentPage.test.tsx` file is the primary new artifact. `utils.test.ts` adds property-based coverage for pure functions. `QueryModal.test.tsx` already exists with good coverage; we extend it only if gaps are identified.

## Components and Interfaces

### AgentPage Test Harness

`AgentPage` depends on:
- `useI18n` / `getCatalog` — provided by wrapping in `<I18nProvider>`
- `api.agentDemo` — mocked via `vi.mock('../lib/api')`
- No router dependency (no `<Link>` or `useNavigate` calls)

```tsx
// Minimal render helper
function renderAgentPage() {
  return render(
    <I18nProvider initialLocale="en">
      <AgentPage />
    </I18nProvider>
  );
}
```

### AgentJob Fixture

A reusable fixture that satisfies the full `AgentJob` interface:

```ts
const mockAgentJob: AgentJob = {
  success: true,
  demo: true,
  jobId: 'job-1',
  query: 'best yield',
  report: {
    topOpportunity: {
      protocol: 'Aave',
      vault: 'USDC Vault',
      chain: 'Ethereum',
      apy: 8.5,
      riskLevel: 'Low',
      whaleConfidence: 'High',
      sentimentScore: 'Bullish',
    },
    reasoning: 'Strong fundamentals.',
    alternatives: ['Compound', 'Yearn'],
    warnings: ['Smart contract risk'],
    rawAnalysis: 'Full analysis text.',
  },
  payments: {
    humanPaid: 1,
    currency: 'USDC',
    network: 'Stellar',
    sellerPayments: [
      { seller: 'seller-1', type: 'yield-data', amount: 0.035, txHash: 'tx-1', onChain: false },
    ],
    totalSpent: 0.14,
    agentProfit: 0.86,
  },
  meta: {
    agentWallet: 'GAGENT',
    timestamp: new Date().toISOString(),
    datasetsQueried: 4,
  },
};
```

### localizeScale Extraction

`localizeScale` is currently defined as a closure inside `AgentPage`. To make it independently testable as a pure function, it will be extracted to `frontend/src/lib/agentUtils.ts`:

```ts
// frontend/src/lib/agentUtils.ts
export function localizeScale(value: string, t: (key: string) => string): string {
  const normalized = value.toLowerCase();
  if (normalized === 'low') return t('agent.scales.low');
  if (normalized === 'medium') return t('agent.scales.medium');
  if (normalized === 'high') return t('agent.scales.high');
  if (normalized === 'neutral') return t('agent.scales.neutral');
  if (normalized === 'bullish') return t('agent.scales.bullish');
  if (normalized === 'bearish') return t('agent.scales.bearish');
  return value;
}
```

`AgentPage.tsx` is updated to import and call this function, keeping behavior identical.

## Data Models

### Test Data Flow

```
vi.mock('../lib/api')
       │
       ▼
vi.mocked(api.agentDemo).mockResolvedValueOnce(mockAgentJob)
       │
       ▼
renderAgentPage()  →  fireEvent (type query, click Run Agent)
       │
       ▼
waitFor(() => screen.getByText('Top Opportunity'))
       │
       ▼
assertions on DOM nodes
```

### Property Test Data Flow (fast-check)

```
fc.property(
  fc.float({ min: 0.0001, max: 1_000_000, noNaN: true }),
  (amount) => {
    const result = formatUSDC(amount, 'en');
    return result.includes('.');
  }
)
```

## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system — essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*


### Property 1: AgentJob result fields are fully rendered

*For any* `AgentJob` response from `api.agentDemo`, the rendered `AgentPage` DOM should contain the protocol name, vault name, APY value, risk level, whale confidence, sentiment score, reasoning text, and all alternative/warning strings from the job.

**Validates: Requirements 3.1, 3.2**

---

### Property 2: All seller payments appear in the payment trail

*For any* `AgentJob` with N seller payments, the rendered payment trail section should contain exactly N payment rows, each showing the seller's amount and data type badge.

**Validates: Requirements 3.3**

---

### Property 3: localizeScale produces non-empty output for all known values and locales

*For any* known scale value in {"Low", "Medium", "High", "Neutral", "Bullish", "Bearish"} and any supported locale in {"en", "es", "fr", "sw"}, `localizeScale` should return a non-empty string.

**Validates: Requirements 4.1**

---

### Property 4: localizeScale is a passthrough for unrecognized values

*For any* string not in the set of known scale values, `localizeScale` should return the original string unchanged.

**Validates: Requirements 4.2**

---

### Property 5: localizeScale is case-insensitive

*For any* known scale value, calling `localizeScale` with the lowercase, uppercase, and title-case variants should all produce the same result.

**Validates: Requirements 4.3**

---

### Property 6: formatUSDC always produces a decimal string

*For any* finite positive number, `formatUSDC` should return a string that contains a decimal point (`.`).

**Validates: Requirements 7.1**

---

### Property 7: truncateAddress never lengthens the input

*For any* string, `truncateAddress` should return a string whose length is less than or equal to the length of the input.

**Validates: Requirements 7.2**

---

### Property 8: truncateAddress ellipsis and prefix for long addresses

*For any* string with length greater than 12, `truncateAddress` should return a string that starts with the first 6 characters of the input and contains "...".

**Validates: Requirements 7.3**

---

## Error Handling

- All API calls in tests are mocked; network errors are simulated via `mockRejectedValueOnce`.
- The `AgentPage` error state is triggered by rejecting `api.agentDemo`; tests assert the error message text and retry button presence.
- The `QueryModal` error state is already tested in the existing `QueryModal.test.tsx`.
- Edge case: `api.agentDemo` rejecting with a non-`Error` value (e.g., a plain string) — `AgentPage` falls back to `t("common.states.error")`. This is tested as an edge-case example.
- `localizeScale` with an empty string should return the empty string (passthrough behavior, covered by Property 4).

## Testing Strategy

### Tooling

| Tool | Purpose |
|------|---------|
| Vitest | Test runner (already configured) |
| @testing-library/react | Component rendering and DOM queries |
| happy-dom | DOM environment (already configured) |
| fast-check | Property-based testing library |

**fast-check** must be added as a dev dependency:
```
npm install --save-dev fast-check
```

### Dual Testing Approach

Unit tests (examples) and property tests are complementary:

- **Unit/example tests** cover specific interactions, step transitions, loading states, and error states where the exact input/output is known.
- **Property tests** cover pure functions and rendering completeness across a range of generated inputs.

### File Layout

```
frontend/src/
├── lib/
│   ├── agentUtils.ts          ← extracted localizeScale (new)
│   └── utils.test.ts          ← property tests for formatUSDC, truncateAddress (new)
└── pages/
    └── AgentPage.test.tsx     ← example + property tests for AgentPage (new)
```

`QueryModal.test.tsx` already covers Requirements 5 and 6 adequately. No new file is needed for QueryModal.

### Property Test Configuration

- Each property test runs a minimum of **100 iterations** via fast-check's default runner.
- Each property test is annotated with a comment referencing the design property number.
- Tag format: `// Feature: frontend-testing-infrastructure, Property N: <property_text>`

### Example: Property Test Structure

```ts
import fc from 'fast-check';

// Feature: frontend-testing-infrastructure, Property 6: formatUSDC always produces a decimal string
it('formatUSDC always contains a decimal point', () => {
  fc.assert(
    fc.property(
      fc.float({ min: 0.0001, max: 1_000_000, noNaN: true }),
      (amount) => formatUSDC(amount, 'en').includes('.')
    ),
    { numRuns: 100 }
  );
});
```

### Unit Test Balance

- Unit tests focus on: step transitions, keyboard events, API mock interactions, conditional rendering (demo badge, empty alternatives/warnings).
- Property tests focus on: pure function correctness across all valid inputs, rendering completeness for arbitrary `AgentJob` fixtures.
- Avoid duplicating what `QueryModal.test.tsx` already covers.
