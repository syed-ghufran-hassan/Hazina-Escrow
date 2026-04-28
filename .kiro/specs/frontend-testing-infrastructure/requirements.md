# Requirements Document

## Introduction

The frontend React/TypeScript application currently has partial test coverage — some components and utilities have tests, but `AgentPage.tsx` (the AI agent interaction page) has no tests, and there is no consistent testing strategy across the codebase. This spec formalizes the testing infrastructure and fills the coverage gaps, focusing on complex interactions, AI response formatting, and regression prevention for key components.

## Glossary

- **AgentPage**: The React page component at `frontend/src/pages/AgentPage.tsx` that handles AI agent queries, displays structured results (top opportunity, reasoning, alternatives, warnings, payment trail), and manages loading/error states.
- **QueryModal**: The multi-step modal component at `frontend/src/components/ui/QueryModal.tsx` that handles dataset purchase flow (details → payment → verifying → result/error).
- **Test_Suite**: The collection of Vitest test files covering the frontend application.
- **RTL**: React Testing Library — the testing library already installed in the project (`@testing-library/react`).
- **Vitest**: The test runner already configured in `frontend/vitest.config.ts`.
- **I18nProvider**: The internationalization context provider required to wrap components under test.
- **AgentJob**: The TypeScript interface representing a complete AI agent response including `report`, `payments`, and `meta` fields.
- **localizeScale**: The pure function inside `AgentPage` that maps English scale values (Low/Medium/High/Bullish/Bearish/Neutral) to localized strings.

## Requirements

### Requirement 1: AgentPage Query Submission

**User Story:** As a developer, I want tests for the AgentPage query submission flow, so that regressions in the AI query input and validation logic are caught automatically.

#### Acceptance Criteria

1. WHEN the query input contains fewer than 5 non-whitespace characters, THE Test_Suite SHALL verify that the Run Agent button is disabled.
2. WHEN the query input contains 5 or more non-whitespace characters, THE Test_Suite SHALL verify that the Run Agent button is enabled.
3. WHEN the Enter key is pressed in the query input, THE Test_Suite SHALL verify that `api.agentDemo` is called with the trimmed query string.
4. WHEN an example query chip is clicked, THE Test_Suite SHALL verify that the query input value is updated to that chip's text.

### Requirement 2: AgentPage Loading and Error States

**User Story:** As a developer, I want tests for AgentPage loading and error handling, so that users always see appropriate feedback during and after API calls.

#### Acceptance Criteria

1. WHEN `api.agentDemo` is pending, THE Test_Suite SHALL verify that the loading spinner and loading label are visible and the result section is absent.
2. WHEN `api.agentDemo` rejects with an error, THE Test_Suite SHALL verify that the error message is displayed and the retry button is present.
3. WHEN the retry button is clicked after an error, THE Test_Suite SHALL verify that `api.agentDemo` is called again with the same query.
4. IF `api.agentDemo` rejects with a non-Error value, THEN THE Test_Suite SHALL verify that a fallback error string is displayed.

### Requirement 3: AgentPage Result Rendering

**User Story:** As a developer, I want tests for AgentPage result display, so that AI response data is always rendered correctly and regressions in formatting are caught.

#### Acceptance Criteria

1. WHEN `api.agentDemo` resolves with an `AgentJob`, THE Test_Suite SHALL verify that the top opportunity fields (protocol, vault, chain, APY, risk level, whale confidence, sentiment) are all visible in the DOM.
2. WHEN `api.agentDemo` resolves with an `AgentJob`, THE Test_Suite SHALL verify that the reasoning text, alternatives list, and warnings list are rendered.
3. WHEN `api.agentDemo` resolves with an `AgentJob`, THE Test_Suite SHALL verify that the payment trail section shows each seller payment with its amount and type badge.
4. WHEN the `demo` flag is true on the result, THE Test_Suite SHALL verify that the "Demo" badge is visible.

### Requirement 4: AgentPage Scale Localization

**User Story:** As a developer, I want property tests for the `localizeScale` function, so that all risk/sentiment/confidence values are correctly mapped for every supported locale.

#### Acceptance Criteria

1. THE Test_Suite SHALL verify that `localizeScale` maps "Low", "Medium", "High", "Neutral", "Bullish", and "Bearish" to non-empty localized strings for every supported locale.
2. WHEN `localizeScale` receives an unrecognized value, THE Test_Suite SHALL verify that the original value is returned unchanged.
3. THE Test_Suite SHALL verify that `localizeScale` output is case-insensitive with respect to the input (i.e., "low", "LOW", and "Low" all produce the same result).

### Requirement 5: QueryModal Step Navigation

**User Story:** As a developer, I want tests for QueryModal multi-step navigation, so that the payment flow progression is always correct.

#### Acceptance Criteria

1. WHEN the modal is first rendered, THE Test_Suite SHALL verify that the details step is active and the "Proceed to Payment" button is present.
2. WHEN "Proceed to Payment" is clicked, THE Test_Suite SHALL verify that the payment step becomes active and `api.initiateQuery` is called.
3. WHEN the "Back" button is clicked on the payment step, THE Test_Suite SHALL verify that the details step is restored.
4. WHEN the Escape key is pressed on the details or payment step, THE Test_Suite SHALL verify that `onClose` is called.

### Requirement 6: QueryModal Result and Error States

**User Story:** As a developer, I want tests for QueryModal terminal states, so that successful purchases and failures are always handled correctly.

#### Acceptance Criteria

1. WHEN `api.demoQuery` resolves successfully, THE Test_Suite SHALL verify that the result step shows the AI summary, transaction breakdown, and raw data preview.
2. WHEN `api.demoQuery` resolves successfully, THE Test_Suite SHALL verify that `onSuccess` is called with updated `queriesServed` and `totalEarned` values.
3. WHEN `api.demoQuery` rejects, THE Test_Suite SHALL verify that the error step is shown with the error message and a "Try Again" button.
4. WHEN "Try Again" is clicked on the error step, THE Test_Suite SHALL verify that the payment step is restored.

### Requirement 7: Utility Function Correctness

**User Story:** As a developer, I want property tests for utility functions, so that formatting and address truncation are always correct across all valid inputs.

#### Acceptance Criteria

1. THE Test_Suite SHALL verify that `formatUSDC` always returns a string containing a decimal point for any finite positive number.
2. THE Test_Suite SHALL verify that `truncateAddress` always returns a string no longer than the original input for any address string.
3. WHEN `truncateAddress` receives an address longer than 12 characters, THE Test_Suite SHALL verify that the result contains "..." and starts with the first 6 characters of the input.
4. THE Test_Suite SHALL verify that `formatUSDC` output for the same number is consistent across repeated calls (idempotent).
