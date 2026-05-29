# Datadog Custom Metrics Documentation

This guide documents all custom domain metrics instrumented in the Hazina Escrow system. These metrics are automatically sent to Datadog when `DATADOG_ENABLED=true` and provide real-time visibility into business operations.

## Quick Start

### Enable Datadog Metrics

Set these environment variables:

```bash
DATADOG_ENABLED=true
DATADOG_SERVICE=hazina-escrow-api
NODE_ENV=production  # or development
DATADOG_VERSION=1.0.0
```

### View Metrics in Datadog

1. Go to [Datadog Metrics Explorer](https://app.datadoghq.com/metric/explorer)
2. Search for metric names (e.g., `payments.verified`, `agent.jobs.completed`)
3. Create dashboards using these metrics
4. Set up alerts for business KPIs

---

## Payment Metrics

### `stellar.payment.verified`

Tracks Stellar payment verification attempts, successes, and specific failure reasons.

**Tags:**

- `dataset_type` - Dataset type being purchased
- `mode` - `real` (on-chain) or `demo` (simulated)
- `status` - `verified` (success) or `failed` (failed)
- `reason` - Error reason (only when status=failed):
  - `no_payment_found` - No payment to escrow found in tx
  - `no_usdc_found` - Non-USDC payment used (e.g., XLM)
  - `amount_mismatch` - Payment amount doesn't match expected
  - `transaction_expired` - Transaction older than 5 minutes
  - `transaction_not_found` - Transaction not found on Stellar

**Example:** Verify Stellar API issues by checking for `reason:transaction_not_found` spike

```sql
sum:stellar.payment.verified{status:failed,reason:amount_mismatch} by {dataset_type}
```

### `stellar.timeout`

Tracks Stellar Horizon API timeouts (when response takes >10s).

**Tags:**

- `method` - API method called (e.g., `transaction`, `operations`)

**Usage:** Alert if timeouts exceed 5% of verification attempts

```sql
sum:stellar.timeout{} / (sum:stellar.payment.verified{} + sum:stellar.timeout{}) > 0.05
```

### `payments.verified`

End-to-end payment tracking: payment verified AND data delivered successfully.

**Tags:**

- `dataset_type` - Type of dataset purchased
- `mode` - `real` or `demo`
- `status` - `delivered` (successful) or `pending` (queued for retry)

**Example:** Track purchase completion rate by dataset type

```sql
sum:payments.verified{status:delivered} by {dataset_type} / sum:payments.verified{} by {dataset_type}
```

### `payments.delivery.failed`

Tracks when payment was verified but data delivery failed (e.g., AI summary error).

**Tags:**

- `dataset_type` - Type of dataset
- `mode` - `real` or `demo`
- `reason` - Failure category:
  - `ai_error` - AI summary generation failed
  - `delivery_error` - Other delivery system error

**Insight:** If AI errors spike, may indicate Claude API throttling

```sql
sum:payments.delivery.failed{reason:ai_error}
```

### `payments.verification.error`

Tracks payment verification errors (validation failures before attempting Stellar check).

**Tags:**

- `mode` - `real` or `demo`
- `error_type` - Error category (e.g., `invalid_amount`, `tx_not_found`)

**Usage:** Monitor for validation issues that indicate buyer mistakes

```sql
sum:payments.verification.error{} by {error_type}
```

### `delivery.retry.attempt`

Tracks delivery retry attempts when initial delivery fails.

**Tags:**

- `dataset_type` - Dataset type
- `mode` - `real` or `demo`
- `attempt` - Attempt number (1, 2, 3, ...)

**Insight:** If attempt:3 or attempt:4 metrics exist, retries queue is backlogged

```sql
avg:delivery.retry.attempt{} by {attempt}
```

### `delivery.retry.succeeded`

Tracks successful retries (data delivered after previous failure).

**Tags:**

- `dataset_type` - Dataset type
- `mode` - `real` or `demo`

**KPI:** Success rate = `delivery.retry.succeeded / delivery.retry.attempt`

---

## Dataset Metrics

### `datasets.created`

Tracks when sellers create new datasets.

**Tags:**

- `dataset_type` - Classification of dataset
- `price_per_query` - Price in USDC

**Insight:** Monitor seller onboarding and dataset type distribution

```sql
sum:datasets.created{} by {dataset_type}
```

### `datasets.queried`

Tracks individual dataset purchases/queries (the main business transaction).

**Tags:**

- `dataset_type` - Type of dataset queried
- `mode` - `real` or `demo`
- `source` - Who initiated: `buyer` (direct purchase) or `agent` (autonomous agent)

**KPI:** Product demand by dataset type

```sql
sum:datasets.queried{} by {dataset_type, source}
```

**Business Metrics:**

- Real buyer queries: `sum:datasets.queried{mode:real,source:buyer}`
- Agent purchases: `sum:datasets.queried{source:agent}`

### `datasets.stats.queries_served`

Gauge of total queries served by each dataset (updated on each query).

**Tags:**

- `dataset_id` - Internal dataset ID
- `dataset_type` - Dataset type

**Usage:** Identify most popular datasets

```sql
top:datasets.stats.queries_served{by:dataset_id} limit:10
```

### `datasets.stats.total_earned`

Gauge of total USDC earned per dataset (updated on each query).

**Tags:**

- `dataset_id` - Dataset ID
- `dataset_type` - Dataset type

**Insight:** Revenue generation by seller

```sql
sum:datasets.stats.total_earned{} by {dataset_id}
```

### `datasets.count`

Total count of datasets by type (can be sampled periodically).

**Tags:**

- `dataset_type` - Dataset type
- `count` - Number of datasets of this type

---

## Agent Metrics

### `agent.jobs.completed`

Tracks completed research agent jobs (successful end-to-end research).

**Tags:**

- `mode` - `real` (funded wallet) or `demo` (simulation)
- `status` - Currently always `completed`
- `datasets_queried` - Number of datasets purchased for this job

**Related Metrics:**

- `agent.jobs.total_spent` - Total USDC spent by agent on dataset purchases

**KPI:** Agent job volume

```sql
sum:agent.jobs.completed{} by {mode}
```

### `agent.jobs.failed`

Tracks failed research agent jobs.

**Tags:**

- `mode` - `real` or `demo`
- `reason` - Failure category:
  - `payment_verification_failed` - Human's 1 USDC payment invalid
  - `tx_already_used` - Transaction hash already used (duplicate protection)
  - `unknown_error` - Unexpected error during research

**Alert:** If `reason:payment_verification_failed` spikes, check Stellar availability

```sql
sum:agent.jobs.failed{reason:payment_verification_failed}
```

### `agent.human_payment.verified`

Tracks verification of human's 1 USDC payment to fund the agent job.

**Tags:**

- `mode` - `real` or `demo`
- `status` - `verified` (success) or `failed` (rejected)

**Usage:** Separate from stellar.payment.verified to distinguish human vs dataset payments

```sql
sum:agent.human_payment.verified{status:verified} / sum:agent.human_payment.verified{} > 0.95
```

### `agent.dataset.purchase`

Tracks agent purchases of datasets (one metric per dataset purchased during a job).

**Tags:**

- `dataset_type` - Dataset type purchased (e.g., `yield-data`, `whale-wallets`)
- `mode` - `real` or `demo`

**Related Metrics:**

- `agent.dataset.spend` - Amount in USDC spent on this purchase

**Insight:** Which dataset types agents value most

```sql
sum:agent.dataset.purchase{} by {dataset_type}
```

---

## Error & Health Metrics

### `validation.error`

Tracks API validation errors (request body errors, malformed input).

**Tags:**

- `endpoint` - API endpoint (e.g., `/api/query/:id`, `/api/verify/:id`)
- `error_type` - Error category

**Usage:** Monitor for client-side issues or API misuse

```sql
sum:validation.error{} by {endpoint}
```

### `circuit_breaker.trip`

Tracks circuit breaker trips (service degradation protection).

**Tags:**

- `service` - Service name (e.g., `stellar-horizon`)

**Alert:** If circuit breaker trips repeatedly, service dependency is failing

```sql
sum:circuit_breaker.trip{service:stellar-horizon} > 3
```

---

## Common Dashboard Patterns

### Business Overview Dashboard

```
Top-left:    sum:datasets.queried{mode:real,source:buyer} - real buyer purchases
Top-right:   sum:agent.jobs.completed{mode:real} - agent job volume
Bottom-left: sum:payments.verified{status:delivered} - successful payments
Bottom-right: sum:payments.delivery.failed{} - delivery failures
```

### Dataset Health Dashboard

```
Chart 1: datasets.queried by {dataset_type} - demand by type
Chart 2: datasets.stats.total_earned by {dataset_id} - top revenue datasets
Chart 3: payments.verified{status:delivered} / payments.verified{*} - completion rate
Chart 4: payments.delivery.failed by {reason} - failure breakdown
```

### Agent Performance Dashboard

```
Chart 1: agent.jobs.completed{mode:real} - job volume over time
Chart 2: agent.dataset.purchase{} by {dataset_type} - agent buying patterns
Chart 3: agent.jobs.failed by {reason} - failure analysis
Chart 4: agent.human_payment.verified{status:verified} / agent.human_payment.verified - verification rate
```

### Stellar Reliability Dashboard

```
Chart 1: stellar.payment.verified{status:verified} / stellar.payment.verified - success rate
Chart 2: stellar.timeout by {method} - timeout frequency
Chart 3: stellar.payment.verified{status:failed} by {reason} - failure breakdown
Chart 4: circuit_breaker.trip{service:stellar-horizon} - circuit breaker health
```

---

## Alerting Recommendations

### Critical Alerts

```yaml
# Payment verification success rate drops below 90%
Alert: "stellar.payment.verified{status:verified} / stellar.payment.verified > 0.9"
Threshold: 5 min average
Action: Check Stellar Horizon API status

# Delivery failures accumulate
Alert: "sum:delivery.retry.attempt{attempt:>1} over last 1hr"
Threshold: > 50 events
Action: Check AI API and delivery system health

# Agent jobs failing
Alert: "sum:agent.jobs.failed{} > 10 in last 1hr"
Threshold: > 10 events
Action: Investigate payment verification or dataset availability
```

### Performance Alerts

```yaml
# Agent response time degradation
Alert: "avg:stellar.payment.verified.duration > 5s"
Threshold: 10 min average
Action: May indicate Stellar network congestion

# Delivery retry queue growing
Alert: "sum:delivery.retry.attempt{attempt:≥3} / sum:delivery.retry.attempt > 0.1"
Threshold: > 10% require 3+ attempts
Action: Check delivery system resources
```

---

## Metrics API Reference

All metrics are instrumented via `domainMetrics` object in [common/datadog.ts](src/common/datadog.ts).

### Adding New Metrics

```typescript
// In domainMetrics object:
myNewMetric(tags: { myTag: string; myValue: number }) {
  incrementMetric('my.new.metric', 1, {
    my_tag: tags.myTag,  // snake_case in Datadog
    my_value: tags.myValue,
  });
}

// Usage in service:
domainMetrics.myNewMetric({ myTag: 'value', myValue: 42 });
```

### Tag Naming Convention

- Service layer tags use `dataset_type`, `mode`, `status` (lowercase with underscore)
- Tags are automatically converted to lowercase and underscores in Datadog
- Boolean tags should use `status: 'verified' | 'failed'` instead of booleans
- Avoid high-cardinality tags (e.g., wallet addresses, txHashes)

---

## Troubleshooting

### Metrics Not Appearing

1. **Check Datadog is enabled:**

   ```bash
   echo $DATADOG_ENABLED  # should be 'true'
   ```

2. **Check dogstatsd is available:**
   - Metrics require dd-trace's built-in DogStatsD client
   - If using alternative libraries, verify compatibility

3. **Check firewall/networking:**
   - DogStatsD uses UDP port 8125 by default
   - Verify connection to Datadog agent

4. **Check logs for errors:**
   ```bash
   grep -i "datadog.*error\|Failed to submit" logs/*.log
   ```

### Metrics Have Wrong Tags

- Tags are normalized to lowercase with underscores
- Check `formatTags()` in [common/datadog.ts](src/common/datadog.ts)
- Use snake_case for composite names: `dataset_type`, not `datasetType`

### High Cardinality Warning

If Datadog warns about "too many tag values", review:

- Don't use transaction hashes or wallet addresses as tags
- Use categories instead: `reason`, `error_type`, `status`
- Keep tag values to <= 20 unique values per tag

---

## Integration with Monitoring

### Grafana Integration

```yaml
datasource:
  type: datadog
  url: https://api.datadoghq.com
  apiKey: ${DATADOG_API_KEY}

panels:
  - title: Daily Queries
    query: 'sum:datasets.queried{}'
```

### Custom Alerts via Webhooks

Configure Datadog to send alerts to your monitoring system:

- Slack notifications for critical failures
- PagerDuty for on-call escalation
- Custom webhooks for Telegram/Discord

---

## Performance Notes

- Metrics are sent asynchronously via DogStatsD
- Typical overhead: <1ms per metric call
- In high-volume scenarios (100+ queries/sec), consider sampling
- All metrics default to `incrementMetric()` which is fast O(1)

---

## References

- [Datadog Custom Metrics](https://docs.datadoghq.com/metrics/)
- [DogStatsD Documentation](https://docs.datadoghq.com/developers/dogstatsd/)
- [dd-trace Node.js Integration](https://github.com/DataDog/dd-trace-js)
- Source: [common/datadog.ts](src/common/datadog.ts)
