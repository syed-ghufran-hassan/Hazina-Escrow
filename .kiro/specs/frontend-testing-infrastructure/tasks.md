# Implementation Plan: Frontend Testing Infrastructure

## Overview

Add `AgentPage.test.tsx` and `utils.test.ts`, extract `localizeScale` to a testable utility, and install `fast-check` for property-based testing. All work builds incrementally — utilities first, then the page component tests.

## Tasks

- [ ] 1. Install fast-check and extract localizeScale
  - [ ] 1.1 Add fast-check dev dependency
    - Run `npm install --save-dev fast-check` in the `frontend/` directory
    - Verify it appears in `package.json` devDependencies
    - _Requirements: 4.1, 4.2, 4.3, 7.1, 7.2, 7.3_

  - [ ] 1.2 Extract localizeScale to `frontend/src/lib/agentUtils.ts`
    - Create `agentUtils.ts` exporting `localizeScale(value: string, t: (key: string) => string): string`
    - Update `AgentPage.tsx` to import and call `localizeScale` from `agentUtils.ts`
    - Ensure no behavior change — all existing rendering still works
    - _Requirements: 4.1, 4.2, 4.3_

- [ ] 2. Property tests for utility functions
  - [ ] 2.1 Create `frontend/src/lib/utils.test.ts` with property tests for `formatUSDC` and `truncateAddress`
    - Import `fc` from `fast-check`
    - Write property test for `formatUSDC` always containing a decimal point (Property 6)
    - Write property test for `truncateAddress` never lengthening the input (Property 7)
    - Write property test for `truncateAddress` ellipsis and prefix for long addresses (Property 8)
    - Each test annotated with `// Feature: frontend-testing-infrastructure, Property N: ...`
    - _Requirements: 7.1, 7.2, 7.3_

  - [ ]* 2.2 Write property tests for localizeScale in `frontend/src/lib/agentUtils.test.ts`
    - Property 3: non-empty output for all known values × all supported locales
    - Property 4: passthrough for unrecognized values (use `fc.string()` filtered to exclude known values)
    - Property 5: case-insensitivity for known values
    - Each test annotated with `// Feature: frontend-testing-infrastructure, Property N: ...`
    - _Requirements: 4.1, 4.2, 4.3_

- [ ] 3. Checkpoint — run existing tests
  - Ensure all existing tests pass with `npm run test` in `frontend/`
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 4. AgentPage example tests — query input and submission
  - [ ] 4.1 Create `frontend/src/pages/AgentPage.test.tsx` with render helper and mock setup
    - Mock `../lib/api` with `vi.mock`
    - Create `renderAgentPage()` helper wrapping in `<I18nProvider initialLocale="en">`
    - Define `mockAgentJob` fixture matching the full `AgentJob` interface
    - _Requirements: 1.1, 1.2, 1.3, 1.4_

  - [ ] 4.2 Write example tests for query input validation (Requirements 1.1, 1.2)
    - Test: button disabled when input has fewer than 5 non-whitespace characters
    - Test: button enabled when input has 5+ non-whitespace characters
    - _Requirements: 1.1, 1.2_

  - [ ] 4.3 Write example tests for query submission interactions (Requirements 1.3, 1.4)
    - Test: pressing Enter calls `api.agentDemo` with trimmed query
    - Test: clicking an example query chip updates the input value
    - _Requirements: 1.3, 1.4_

- [ ] 5. AgentPage example tests — loading and error states
  - [ ] 5.1 Write example tests for loading state (Requirement 2.1)
    - Use a never-resolving promise to hold the pending state
    - Assert loading spinner is visible and result section is absent
    - _Requirements: 2.1_

  - [ ] 5.2 Write example tests for error state and retry (Requirements 2.2, 2.3, 2.4)
    - Test: error message displayed when `api.agentDemo` rejects with an `Error`
    - Test: retry button present and calls `api.agentDemo` again on click
    - Edge-case test: fallback string shown when rejection value is not an `Error` instance
    - _Requirements: 2.2, 2.3, 2.4_

- [ ] 6. AgentPage tests — result rendering
  - [ ] 6.1 Write example test for demo badge visibility (Requirement 3.4)
    - Mock `api.agentDemo` to resolve with `mockAgentJob` (demo: true)
    - Assert "Demo" badge is in the DOM
    - _Requirements: 3.4_

  - [ ]* 6.2 Write property test for AgentJob result fields rendered (Property 1, Property 2)
    - Use `fc.record(...)` to generate random `AgentJob`-shaped objects
    - Assert all top opportunity fields, reasoning, alternatives, warnings appear in DOM
    - Assert payment trail contains one row per seller payment
    - Annotated: `// Feature: frontend-testing-infrastructure, Property 1` and `Property 2`
    - _Requirements: 3.1, 3.2, 3.3_

- [ ] 7. Final checkpoint — full test suite
  - Run `npm run test` in `frontend/` and ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.
