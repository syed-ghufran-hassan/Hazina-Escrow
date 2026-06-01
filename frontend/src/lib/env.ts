/**
 * Environment variable validation and central access.
 *
 * Call initEnv() once at the very top of app startup (main.tsx) so missing or
 * invalid env vars fail loudly instead of causing cryptic runtime errors.
 */

export interface EnvConfig {
  /** Base URL of the backend API (e.g. http://localhost:3001) */
  apiUrl: string;
  /** API key required for administrative actions like creating datasets */
  apiKey: string;
  /** Stellar network to use: 'testnet' or 'public' (mainnet) */
  stellarNetwork: 'testnet' | 'public';
  /** Optional override for the USDC asset issuer address on Stellar */
  usdcIssuer?: string;
  /** Maximum number of concurrent API requests allowed */
  maxConcurrentRequests: number;
}

const REQUIRED_ENV_VARS = ['VITE_API_URL', 'VITE_API_KEY'] as const;

/**
 * Validate required environment variables and return a typed config object.
 * Throws a descriptive error when a required variable is missing or empty.
 */
export function validateEnv(): EnvConfig {
  const missing: string[] = [];

  // Try import.meta.env first (Vite), fall back to process.env (Node/test environment)
  const envVars = (import.meta.env as Record<string, string | undefined>) || process.env;

  for (const key of REQUIRED_ENV_VARS) {
    const value = envVars[key];
    if (!value || String(value).trim() === '') {
      missing.push(key);
    }
  }

  if (missing.length > 0) {
    const list = missing.map(k => `  • ${k}`).join('\n');
    throw new Error(
      `[Hazina] Missing required environment variable(s):\n${list}\n\n` +
        'Copy .env.example to .env and fill in the missing values before starting the app.',
    );
  }

  return {
    apiUrl: String(envVars.VITE_API_URL).trim().replace(/\/+$/, ''),
    apiKey: String(envVars.VITE_API_KEY).trim(),
    stellarNetwork: ((): 'testnet' | 'public' => {
      const n = (envVars.VITE_STELLAR_NETWORK || 'testnet').trim().toLowerCase();
      return n === 'mainnet' || n === 'public' ? 'public' : 'testnet';
    })(),
    usdcIssuer: envVars.VITE_USDC_ISSUER?.trim(),
    maxConcurrentRequests: (() => {
      const raw = parseInt(envVars.VITE_MAX_CONCURRENT_REQUESTS || '8', 10);
      return Number.isFinite(raw) && raw > 0 ? raw : 8;
    })(),
  };
}

/**
 * Validated env config — available after validateEnv() succeeds.
 * Import this anywhere you need typed access to env values.
 */
let _env: EnvConfig | null = null;

export function getEnv(): EnvConfig {
  if (!_env) {
    throw new Error(
      '[Hazina] getEnv() called before validateEnv(). ' +
        'Make sure validateEnv() is called in main.tsx before rendering the app.',
    );
  }
  return _env;
}

export function initEnv(): EnvConfig {
  _env = validateEnv();
  return _env;
}
