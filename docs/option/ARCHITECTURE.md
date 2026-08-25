# Option Analysis Platform - Architecture

## System Overview

The Option Analysis Platform extends AlphaForge's AlphaForge with professional-grade options analysis capabilities. It follows the existing architectural patterns while adding specialized components for option pricing, Greeks calculation, and strategy analysis.

```text
┌─────────────────────────────────────────────────────────────┐
│                        Tauri 2 Desktop                        │
│                                                               │
│  ┌──────────────────────┐         ┌────────────────────────┐ │
│  │   React Frontend     │◄───────►│     Rust Backend       │ │
│  │   (Presentation)     │   IPC   │   (Capability Layer)   │ │
│  │                      │         │                        │ │
│  │  • Option Pages      │         │  • Option Service      │ │
│  │  • Strategy Builder  │         │  • Pricing Engine      │ │
│  │  • Greeks Display    │         │  • Greeks Calculator   │ │
│  │  • Risk Dashboards   │         │  • Data Providers      │ │
│  └──────────────────────┘         └───────────┬────────────┘ │
│                                               │               │
│                                   ┌───────────▼──────────┐   │
│                                   │   SQLite Database    │   │
│                                   │   (Persistence)      │   │
│                                   └──────────────────────┘   │
│                                                               │
│  ┌────────────────────────────────────────────────────────┐  │
│  │            Artifact WebView Windows (Isolated)         │  │
│  │  • Option Chain Viewer Plugin                          │  │
│  │  • Strategy Payoff Plugin                              │  │
│  │  • Volatility Surface Plugin                           │  │
│  │  • Risk Dashboard Plugin                               │  │
│  └────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## Architectural Layers

### Layer 1: React Frontend (Presentation)

**Responsibilities**:
- User interface rendering
- User interaction handling
- Frontend state management
- IPC communication via `desktopApi`
- Loading/error/empty state display
- Keyboard navigation and accessibility

**Owns**:
- Pages: `apps/desktop/src/pages/options/`
- Feature components: `apps/desktop/src/features/options/`
- IPC client: `apps/desktop/src/lib/desktop-api/options.ts`
- Type definitions: `apps/desktop/src/types/option.ts`
- UI state (TanStack Query, Zustand)

**Must NOT**:
- Access SQLite directly
- Read filesystem or arbitrary files
- Access plaintext API keys
- Invoke shell commands
- Bypass `desktopApi` IPC layer
- Perform option pricing calculations (delegate to backend)

**Integration Points**:
```typescript
// Frontend calls backend through desktopApi
import { desktopApi } from '@/lib/desktop-api';

const optionChain = await desktopApi.options.fetchOptionChain('AAPL');
const greeks = await desktopApi.options.calculateGreeks(params);
```

### Layer 2: Rust Backend (Capability Layer)

**Responsibilities**:
- Option pricing calculations (Black-Scholes, Binomial)
- Greeks calculations (Delta, Gamma, Theta, Vega, Rho)
- Strategy analysis and payoff calculations
- Database operations (CRUD)
- Market data fetching from providers
- API key management (OS keychain)
- Background task execution
- Input validation and sanitization
- Structured logging and telemetry

**Owns**:
- Domain models: `crates/domain/src/option.rs`
- Pricing engine: `crates/option-core/`
- Services: `apps/desktop/src-tauri/src/services/option_service.rs`
- Commands: `apps/desktop/src-tauri/src/commands/options.rs`
- Repositories: `apps/desktop/src-tauri/src/database/repositories/option_*.rs`
- Data providers: `apps/desktop/src-tauri/src/providers/market_data/options_provider.rs`

**Must NOT**:
- Build large HTML strings
- Own page layout
- Manage React state
- Perform pure presentation logic

**Service Architecture**:
```rust
// Service layer orchestrates business logic
pub struct OptionService {
    chain_repo: Arc<OptionChainRepository>,
    contract_repo: Arc<OptionContractRepository>,
    pricing_engine: Arc<dyn OptionPricer>,
    data_provider: Arc<dyn OptionsDataProvider>,
}

impl OptionService {
    pub async fn fetch_chain(&self, symbol: &str) -> Result<OptionChain, AppError> {
        // 1. Fetch data from provider
        // 2. Validate and calculate Greeks
        // 3. Persist to database
        // 4. Return to frontend
    }
}
```

### Layer 3: Tauri (Desktop Runtime)

**Responsibilities**:
- Desktop window management
- IPC communication (React ↔ Rust)
- Permission boundaries and capabilities
- Artifact WebView creation
- Operating system integration
- Application lifecycle
- Auto-update infrastructure

**Owns**:
- Main window: `apps/desktop/src-tauri/tauri.conf.json`
- Artifact windows: Temporary WebViews with restricted permissions
- Capabilities: `apps/desktop/src-tauri/capabilities/`

**Permission Model**:
```json
// capabilities/main-window.json
{
  "identifier": "main-window",
  "permissions": [
    "core:default",
    "opener:default",
    "store:default"
  ]
}

// capabilities/artifact-window.json (for option plugins)
{
  "identifier": "artifact-window",
  "permissions": [
    "core:default"
  ]
}
```

### Layer 4: SQLite Database (Persistence)

**Responsibilities**:
- Persistent storage for option entities
- Transaction support for multi-step operations
- Migration management (append-only)
- Query optimization via indexes

**Schema Design**:
- Workspace-scoped entities
- Foreign key constraints
- Timestamps on all records
- Soft deletion for historical integrity

**Migration Strategy**:
- Migrations are append-only after release
- Never modify applied migrations
- Use `sqlx` for type-safe queries
- Version tracking in `_migrations` table

---

## Directory Structure

### New Directories for Option Platform

```
alpha-forge/
├── crates/
│   ├── domain/
│   │   └── src/
│   │       └── option.rs                  # NEW: Domain models
│   └── option-core/                        # NEW: Pricing & calculations
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── pricing.rs                  # Black-Scholes, Binomial
│           ├── greeks.rs                   # Greeks calculations
│           ├── volatility.rs              # IV solver
│           ├── strategy.rs                # Multi-leg strategy analysis
│           └── models.rs                  # Pricing models
│
├── apps/desktop/
│   ├── src/                                # React frontend
│   │   ├── pages/
│   │   │   └── options/                    # NEW
│   │   │       ├── OptionsDashboard.tsx
│   │   │       ├── OptionChainPage.tsx
│   │   │       ├── StrategyBuilderPage.tsx
│   │   │       ├── VolatilityAnalysisPage.tsx
│   │   │       └── PortfolioRiskPage.tsx
│   │   ├── features/
│   │   │   └── options/                    # NEW
│   │   │       ├── OptionChainViewer/
│   │   │       ├── GreeksCalculator/
│   │   │       ├── StrategyBuilder/
│   │   │       ├── VolatilitySurface/
│   │   │       └── PortfolioRisk/
│   │   ├── lib/
│   │   │   └── desktop-api/
│   │   │       └── options.ts             # NEW: IPC client
│   │   └── types/
│   │       └── option.ts                  # NEW: TypeScript interfaces
│   │
│   └── src-tauri/                          # Rust backend
│       ├── src/
│       │   ├── commands/
│       │   │   └── options.rs             # NEW: IPC commands
│       │   ├── services/
│       │   │   └── option_service.rs      # NEW: Business logic
│       │   ├── database/
│       │   │   └── repositories/
│       │   │       ├── option_chain_repository.rs       # NEW
│       │   │       ├── option_contract_repository.rs    # NEW
│       │   │       ├── greeks_repository.rs             # NEW
│       │   │       ├── option_strategy_repository.rs    # NEW
│       │   │       └── option_position_repository.rs    # NEW
│       │   └── providers/
│       │       └── market_data/
│       │           └── options_provider.rs # NEW: Data fetching
│       └── migrations/
│           ├── 0004_options_support.sql    # HISTORICAL: unchanged
│           └── 0014_options_support.sql    # CANONICAL: registered by migrations.rs
│
└── plugins/                                # Artifact plugins
    ├── option-chain/                       # NEW
    ├── strategy-payoff/                    # NEW
    ├── volatility-surface/                 # NEW
    └── risk-dashboard/                     # NEW
```

### Existing Directories to Extend

```
alpha-forge/
├── packages/
│   └── financial-components/               # EXTEND
│       └── src/
│           ├── options/                    # NEW: Option-specific UI
│           │   ├── PayoffDiagram.tsx
│           │   ├── GreeksTable.tsx
│           │   ├── StrategySelector.tsx
│           │   └── VolatilityChart.tsx
│           └── index.ts
│
└── docs/
    └── option/                             # NEW (created)
        ├── PRODUCT.md                      # ✅ Created
        ├── ROADMAP.md                      # ✅ Created
        ├── USE_CASES.md                    # ✅ Created
        ├── ARCHITECTURE.md                 # ✅ This file
        ├── DATA_MODEL.md                   # 📋 Next
        ├── API_SPEC.md                     # 📋 Next
        ├── IMPLEMENTATION_GUIDE.md         # 📋 Phase 7
        ├── TESTING_STRATEGY.md             # 📋 Phase 7
        ├── DATA_SOURCES.md                 # 📋 Phase 2
        └── PLUGIN_GUIDE.md                 # 📋 Phase 4
```

---

## IPC Communication Flow

### Request-Response Pattern

```text
React Component
    ↓ User action (e.g., "Fetch AAPL chain")
desktopApi.options.fetchOptionChain('AAPL')
    ↓ TypeScript IPC wrapper
invoke('fetch_option_chain', { symbol: 'AAPL' })
    ↓ Tauri IPC bridge
#[tauri::command]
fn fetch_option_chain(symbol: String, state: State<AppState>)
    ↓ Rust command handler
OptionService::fetch_chain(&self, symbol)
    ↓ Business logic
    ├─ OptionsDataProvider::fetch_chain('AAPL')
    ├─ PricingEngine::calculate_greeks(contracts)
    └─ OptionChainRepository::save(chain)
    ↓ Return result
Result<OptionChain, AppError>
    ↓ Serialize to JSON
JSON response
    ↓ Tauri IPC bridge
React receives OptionChain object
    ↓ Update UI
Render option chain table
```

### Event Streaming Pattern (Future)

For long-running calculations or real-time updates:

```rust
// Backend emits progress events
#[tauri::command]
async fn calculate_strategy_payoff(
    strategy_id: String,
    app: AppHandle,
) -> Result<(), AppError> {
    // Emit progress events
    app.emit("strategy_calc_progress", ProgressPayload { percent: 25 })?;
    
    // Perform calculation
    let result = calculate_complex_strategy(&strategy_id).await?;
    
    // Emit completion
    app.emit("strategy_calc_complete", result)?;
    
    Ok(())
}
```

```typescript
// Frontend listens for events
import { listen } from '@tauri-apps/api/event';

const unlisten = await listen('strategy_calc_progress', (event) => {
  setProgress(event.payload.percent);
});
```

---

## Data Provider Architecture

### Provider Trait

```rust
// apps/desktop/src-tauri/src/providers/market_data/options_provider.rs

#[async_trait]
pub trait OptionsDataProvider: Send + Sync {
    /// Fetch option chain for a symbol
    async fn fetch_chain(&self, symbol: &str) -> Result<OptionChain, ProviderError>;
    
    /// Fetch quotes for specific contracts
    async fn fetch_quotes(
        &self,
        contracts: &[String]
    ) -> Result<Vec<OptionQuote>, ProviderError>;
    
    /// Provider name for logging and UI
    fn name(&self) -> &str;
    
    /// Check if provider is available
    fn is_available(&self) -> bool;
}
```

### Provider Implementations

#### 1. Demo Provider (Phase 2)

```rust
pub struct DemoOptionsProvider {
    config: DemoConfig,
}

impl DemoOptionsProvider {
    /// Generate realistic option chain using Black-Scholes
    fn generate_chain(&self, symbol: &str, spot_price: f64) -> OptionChain {
        // Generate strikes around spot price
        // Calculate theoretical prices with configurable IV
        // Add realistic bid/ask spreads
        // Return simulated chain
    }
}
```

**Use Case**: Development, testing, education, no API key required

#### 2. File Provider (Phase 2)

```rust
pub struct FileOptionsProvider;

impl FileOptionsProvider {
    /// Parse CSV or JSON file with option data
    async fn parse_file(&self, path: &Path) -> Result<OptionChain, ProviderError>;
    
    /// Supported formats: CSV, JSON
    fn supported_formats() -> Vec<&'static str> {
        vec!["csv", "json"]
    }
}
```

**Use Case**: Historical data analysis, custom scenarios, data from other sources

#### 3. Live Provider (Phase 2, stub)

```rust
pub struct LiveOptionsProvider {
    api_key: SecureString,  // Stored in OS keychain
    client: reqwest::Client,
}

impl LiveOptionsProvider {
    /// Fetch live data from market data API
    async fn fetch_from_api(&self, symbol: &str) -> Result<ApiResponse, ProviderError>;
}
```

**Use Case**: Real market data, requires API key and subscription

**API Key Security**:
```rust
// API key stored in OS keychain, never exposed to React
use keyring::Entry;

fn store_api_key(provider: &str, key: &str) -> Result<(), AppError> {
    let entry = Entry::new("alphaforge", provider)?;
    entry.set_password(key)?;
    Ok(())
}

fn get_api_key(provider: &str) -> Result<String, AppError> {
    let entry = Entry::new("alphaforge", provider)?;
    let key = entry.get_password()?;
    Ok(key)
}
```

### Provider Factory

```rust
pub struct ProviderFactory {
    config: AppConfig,
}

impl ProviderFactory {
    pub fn create_provider(&self, source_type: DataSource) -> Arc<dyn OptionsDataProvider> {
        match source_type {
            DataSource::Demo => Arc::new(DemoOptionsProvider::new(self.config.demo)),
            DataSource::File => Arc::new(FileOptionsProvider),
            DataSource::Live => {
                let api_key = get_api_key("polygon")?;
                Arc::new(LiveOptionsProvider::new(api_key))
            }
        }
    }
}
```

### Fallback Chain

```rust
pub struct ResilientProvider {
    primary: Arc<dyn OptionsDataProvider>,
    fallback: Arc<dyn OptionsDataProvider>,
}

impl OptionsDataProvider for ResilientProvider {
    async fn fetch_chain(&self, symbol: &str) -> Result<OptionChain, ProviderError> {
        match self.primary.fetch_chain(symbol).await {
            Ok(chain) => Ok(chain),
            Err(e) => {
                warn!("Primary provider failed, using fallback: {}", e);
                self.fallback.fetch_chain(symbol).await
            }
        }
    }
}
```

---

## Plugin Architecture

### Plugin Structure

Each option plugin follows the standard AlphaForge plugin specification:

```
plugins/option-chain/
├── manifest.json          # Plugin metadata
├── schema.json            # Input JSON schema
├── index.html             # Entry point for WebView
├── dist/                  # Built React components
│   └── bundle.js
└── src/
    ├── App.tsx
    ├── components/
    │   ├── ChainTable.tsx
    │   ├── Filters.tsx
    │   └── GreeksDisplay.tsx
    └── types.ts
```

### Option Chain Plugin Example

**manifest.json**:
```json
{
  "id": "option-chain",
  "name": "Option Chain Viewer",
  "version": "0.1.0",
  "entry": "index.html",
  "inputSchema": "schema.json",
  "permissions": [],
  "window": {
    "width": 1200,
    "height": 800,
    "resizable": true
  }
}
```

**schema.json**:
```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["chainId", "symbol", "underlyingPrice", "contracts"],
  "properties": {
    "chainId": { "type": "string" },
    "symbol": { "type": "string" },
    "underlyingPrice": { "type": "number" },
    "asOf": { "type": "string", "format": "date-time" },
    "contracts": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["strike", "type", "bid", "ask"],
        "properties": {
          "strike": { "type": "number" },
          "type": { "enum": ["call", "put"] },
          "bid": { "type": "number" },
          "ask": { "type": "number" },
          "volume": { "type": "integer" },
          "openInterest": { "type": "integer" },
          "greeks": {
            "type": "object",
            "properties": {
              "delta": { "type": "number" },
              "gamma": { "type": "number" },
              "theta": { "type": "number" },
              "vega": { "type": "number" },
              "rho": { "type": "number" },
              "iv": { "type": "number" }
            }
          }
        }
      }
    }
  }
}
```

### Plugin Security Model

**Artifact Window Restrictions**:
- ❌ No filesystem access
- ❌ No network access
- ❌ No API keys or credentials
- ❌ No shell execution
- ❌ No SQLite access
- ❌ No access to main window DOM or state

**Allowed**:
- ✅ Receive validated JSON input from agent
- ✅ Render interactive UI
- ✅ Communicate with main window via narrow message protocol
- ✅ Request export (triggers main window save dialog)

**Message Protocol**:
```typescript
// Artifact → Main Window
type ArtifactMessage =
  | { type: 'close' }
  | { type: 'persist'; data: any }
  | { type: 'action'; payload: { type: string; data: any } };

// Main Window → Artifact
type MainWindowMessage =
  | { type: 'update'; data: any }
  | { type: 'theme'; mode: 'light' | 'dark' };
```

---

## Performance Architecture

### Performance Requirements

| Operation | Target | Measurement Method |
|-----------|--------|-------------------|
| Option chain load (100 options) | < 2s | End-to-end timing from request to render |
| Single option pricing | < 10μs | Micro-benchmark with Criterion |
| Single option Greeks | < 50μs | Micro-benchmark with Criterion |
| Chain Greeks (100 options) | < 100ms | Integration test timing |
| Volatility surface interpolation | < 500ms | Performance test with synthetic data |
| Payoff diagram render | < 500ms | React Profiler + manual measurement |
| 3D surface render | 30+ FPS | Frame timing in WebGL |

### Optimization Strategies

#### Backend (Rust)

**1. Parallel Calculation**:
```rust
use rayon::prelude::*;

pub fn calculate_chain_greeks(contracts: &[OptionContract]) -> Vec<Greeks> {
    contracts.par_iter()
        .map(|contract| calculate_greeks(contract))
        .collect()
}
```

**2. Caching**:
```rust
use lru::LruCache;
use std::sync::Mutex;

pub struct CachedPricingEngine {
    cache: Mutex<LruCache<PricingKey, f64>>,
    engine: BlackScholesEngine,
}

impl CachedPricingEngine {
    pub fn price(&self, params: &PricingParams) -> Result<f64, PricingError> {
        let key = params.to_key();

        let mut cache = self
            .cache
            .lock()
            .map_err(|_| PricingError::CacheUnavailable)?;
        if let Some(&price) = cache.get(&key) {
            return Ok(price);
        }

        let price = self.engine.price(params);
        cache.put(key, price);
        Ok(price)
    }
}
```

**3. Lazy Evaluation**:
```rust
pub struct OptionChain {
    contracts: Vec<OptionContract>,
    greeks: OnceCell<Vec<Greeks>>,  // Calculate only when needed
}

impl OptionChain {
    pub fn get_greeks(&self) -> &Vec<Greeks> {
        self.greeks.get_or_init(|| {
            calculate_chain_greeks(&self.contracts)
        })
    }
}
```

#### Frontend (React)

**1. Virtual Scrolling**:
```typescript
// For large option chains
import { FixedSizeList } from 'react-window';

<OptionChainList>
  <FixedSizeList
    height={600}
    itemCount={contracts.length}
    itemSize={35}
  >
    {({ index, style }) => (
      <OptionRow contract={contracts[index]} style={style} />
    )}
  </FixedSizeList>
</OptionChainList>
```

**2. Memoization**:
```typescript
import { memo, useMemo } from 'react';

const OptionRow = memo(({ contract }: OptionRowProps) => {
  // Expensive rendering logic
}, (prev, next) => {
  // Custom comparison to prevent unnecessary re-renders
  return prev.contract.id === next.contract.id &&
         prev.contract.bid === next.contract.bid;
});

const GreeksDisplay = ({ greeks }: GreeksProps) => {
  const formattedGreeks = useMemo(() => {
    return formatGreeks(greeks);
  }, [greeks]);
  
  return <div>{formattedGreeks}</div>;
};
```

**3. Web Workers** (Future):
```typescript
// Move heavy calculations to worker thread
const worker = new Worker('option-calculator.js');

worker.postMessage({ type: 'calculate_greeks', params });
worker.onmessage = (e) => {
  setGreeks(e.data.result);
};
```

---

## Error Handling Architecture

### Error Types

```rust
// crates/shared/src/error.rs

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Option pricing error: {0}")]
    Pricing(String),
    
    #[error("Invalid option parameters: {0}")]
    InvalidParams(String),
    
    #[error("Data provider error: {0}")]
    Provider(#[from] ProviderError),
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Greeks calculation failed: {0}")]
    GreeksCalculation(String),
    
    #[error("Strategy analysis error: {0}")]
    Strategy(String),
}

#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    #[error("API request failed: {0}")]
    ApiRequest(String),
    
    #[error("Invalid API key")]
    InvalidApiKey,
    
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    
    #[error("No data available for symbol: {0}")]
    NoData(String),
    
    #[error("File parse error: {0}")]
    FileParse(String),
}
```

### Error Response Format

```rust
#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    pub context: Option<serde_json::Value>,
    pub recoverable: bool,
}

impl From<AppError> for ErrorResponse {
    fn from(error: AppError) -> Self {
        match error {
            AppError::InvalidParams(msg) => ErrorResponse {
                code: "INVALID_PARAMS".to_string(),
                message: msg,
                context: None,
                recoverable: false,
            },
            AppError::Provider(ProviderError::RateLimitExceeded) => ErrorResponse {
                code: "RATE_LIMIT".to_string(),
                message: "Rate limit exceeded, please try again later".to_string(),
                context: Some(json!({ "retryAfter": 60 })),
                recoverable: true,
            },
            // ... other error mappings
        }
    }
}
```

### Frontend Error Handling

```typescript
// apps/desktop/src/lib/desktop-api/options.ts

export class OptionApiError extends Error {
  constructor(
    public code: string,
    message: string,
    public recoverable: boolean,
    public context?: any
  ) {
    super(message);
    this.name = 'OptionApiError';
  }
}

export async function fetchOptionChain(symbol: string): Promise<OptionChain> {
  try {
    return await invoke<OptionChain>('fetch_option_chain', { symbol });
  } catch (error) {
    const errorResponse = error as ErrorResponse;
    throw new OptionApiError(
      errorResponse.code,
      errorResponse.message,
      errorResponse.recoverable,
      errorResponse.context
    );
  }
}

// Usage in component
const { data, error } = useQuery({
  queryKey: ['optionChain', symbol],
  queryFn: () => fetchOptionChain(symbol),
  retry: (failureCount, error) => {
    if (error instanceof OptionApiError && error.recoverable) {
      return failureCount < 3;
    }
    return false;
  }
});
```

---

## Security Architecture

### API Key Management

```rust
// apps/desktop/src-tauri/src/security/credentials.rs

use keyring::Entry;

pub struct CredentialManager;

impl CredentialManager {
    /// Store API key in OS keychain
    pub fn store_provider_key(provider: &str, key: &str) -> Result<(), AppError> {
        let entry = Entry::new("alphaforge", provider)
            .map_err(|e| AppError::Security(e.to_string()))?;
        
        entry.set_password(key)
            .map_err(|e| AppError::Security(e.to_string()))?;
        
        info!("API key stored for provider: {}", provider);
        Ok(())
    }
    
    /// Retrieve API key from OS keychain
    pub fn get_provider_key(provider: &str) -> Result<String, AppError> {
        let entry = Entry::new("alphaforge", provider)
            .map_err(|e| AppError::Security(e.to_string()))?;
        
        let key = entry.get_password()
            .map_err(|e| AppError::Security(e.to_string()))?;
        
        Ok(key)
    }
    
    /// Delete API key from OS keychain
    pub fn delete_provider_key(provider: &str) -> Result<(), AppError> {
        let entry = Entry::new("alphaforge", provider)
            .map_err(|e| AppError::Security(e.to_string()))?;
        
        entry.delete_credential()
            .map_err(|e| AppError::Security(e.to_string()))?;
        
        info!("API key deleted for provider: {}", provider);
        Ok(())
    }
}
```

**Security Guarantees**:
- ✅ API keys never exposed to React frontend
- ✅ Keys stored in OS-specific secure storage:
  - Windows: Windows Credential Manager
  - macOS: Keychain
  - Linux: Secret Service API
- ✅ Keys only accessible to Rust backend
- ✅ No keys in logs or error messages

### Input Validation

```rust
// apps/desktop/src-tauri/src/security/validation.rs

pub fn validate_symbol(symbol: &str) -> Result<(), AppError> {
    // Prevent injection attacks
    if symbol.len() > 10 {
        return Err(AppError::InvalidParams("Symbol too long".to_string()));
    }
    
    if !symbol.chars().all(|c| c.is_ascii_uppercase()) {
        return Err(AppError::InvalidParams("Invalid symbol format".to_string()));
    }
    
    Ok(())
}

pub fn validate_strike(strike: f64) -> Result<(), AppError> {
    if strike <= 0.0 || strike > 1_000_000.0 {
        return Err(AppError::InvalidParams("Invalid strike price".to_string()));
    }
    
    Ok(())
}

pub fn validate_expiration(expiration: DateTime<Utc>) -> Result<(), AppError> {
    if expiration <= Utc::now() {
        return Err(AppError::InvalidParams("Expiration must be in the future".to_string()));
    }
    
    if expiration > Utc::now() + Duration::days(365 * 3) {
        return Err(AppError::InvalidParams("Expiration too far in the future".to_string()));
    }
    
    Ok(())
}
```

### Data Sanitization

```rust
// Redact sensitive data from logs
pub fn sanitize_for_logging(data: &serde_json::Value) -> serde_json::Value {
    let mut sanitized = data.clone();
    
    if let Some(obj) = sanitized.as_object_mut() {
        // Redact known sensitive fields
        for key in ["api_key", "apiKey", "password", "token", "secret"] {
            if obj.contains_key(key) {
                obj.insert(key.to_string(), json!("[REDACTED]"));
            }
        }
    }
    
    sanitized
}
```

---

## Testing Architecture

### Test Pyramid

```text
        ┌─────────────┐
        │   E2E Tests │  (Critical user flows)
        │    5-10%    │
        ├─────────────┤
        │ Integration │  (IPC, database, providers)
        │    20-30%   │
        ├─────────────┤
        │  Unit Tests │  (Pricing, Greeks, domain logic)
        │    60-70%   │
        └─────────────┘
```

### Rust Test Categories

**1. Domain Model Tests**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_option_contract_creation() {
        let contract = OptionContract {
            id: Uuid::new_v4().to_string(),
            workspace_id: "ws-1".to_string(),
            symbol: "AAPL".to_string(),
            option_type: OptionType::Call,
            strike: 150.0,
            expiration: Utc::now() + Duration::days(30),
            contract_multiplier: 100,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        assert_eq!(contract.symbol, "AAPL");
        assert_eq!(contract.strike, 150.0);
    }
}
```

**2. Pricing Engine Tests**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_black_scholes_call() {
        // Test against known values from financial literature
        let price = black_scholes_price(
            OptionType::Call,
            100.0,  // S - Stock price
            100.0,  // K - Strike
            1.0,    // T - Time to expiration (years)
            0.05,   // r - Risk-free rate
            0.2,    // sigma - Volatility
            0.0     // q - Dividend yield
        );
        
        // Expected: $10.4506 (from Black-Scholes formula)
        assert!((price - 10.4506).abs() < 0.001);
    }
    
    #[test]
    fn test_greeks_delta() {
        let delta = calculate_delta(
            OptionType::Call,
            100.0,
            100.0,
            1.0,
            0.05,
            0.2,
            0.0
        );
        
        // ATM call should have delta ≈ 0.5
        assert!((delta - 0.5).abs() < 0.05);
    }
}
```

**3. Repository Tests**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;
    
    #[sqlx::test]
    async fn test_save_option_chain(pool: SqlitePool) {
        let repo = OptionChainRepository::new(pool);
        
        let chain = OptionChain {
            id: "chain-1".to_string(),
            workspace_id: "ws-1".to_string(),
            symbol: "AAPL".to_string(),
            underlying_price: 150.0,
            as_of: Utc::now(),
            created_at: Utc::now(),
        };
        
        repo.save(&chain).await.unwrap();
        
        let loaded = repo.find_by_id("chain-1").await.unwrap();
        assert_eq!(loaded.symbol, "AAPL");
    }
}
```

### TypeScript Test Categories

**1. Component Tests**:
```typescript
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { OptionChainTable } from './OptionChainTable';

describe('OptionChainTable', () => {
  it('displays option chain data', () => {
    const chain = createMockChain();
    
    render(<OptionChainTable data={chain} />);
    
    expect(screen.getByText('AAPL')).toBeInTheDocument();
    expect(screen.getByText('150.00')).toBeInTheDocument();
  });
  
  it('shows loading state', () => {
    render(<OptionChainTable data={null} isLoading />);
    
    expect(screen.getByTestId('skeleton-loader')).toBeInTheDocument();
  });
  
  it('shows empty state', () => {
    render(<OptionChainTable data={null} isLoading={false} />);
    
    expect(screen.getByText('No options available')).toBeInTheDocument();
  });
});
```

**2. Hook Tests**:
```typescript
import { renderHook, waitFor } from '@testing-library/react';
import { useOptionChain } from './useOptionChain';

describe('useOptionChain', () => {
  it('fetches option chain', async () => {
    const { result } = renderHook(() => useOptionChain('AAPL'));
    
    await waitFor(() => expect(result.current.isSuccess).toBe(true));
    
    expect(result.current.data?.symbol).toBe('AAPL');
  });
});
```

### E2E Test Scenarios

**Critical Flow 1: Load Option Chain**:
```typescript
// tests/e2e/option-chain.spec.ts
import { test, expect } from '@playwright/test';

test('load and display option chain', async ({ page }) => {
  await page.goto('/app/options/chain');
  
  // Enter symbol
  await page.fill('[data-testid="symbol-input"]', 'AAPL');
  await page.press('[data-testid="symbol-input"]', 'Enter');
  
  // Wait for chain to load
  await expect(page.locator('[data-testid="chain-table"]')).toBeVisible();
  
  // Verify Greeks displayed
  await expect(page.locator('[data-testid="greeks-delta"]')).toBeVisible();
});
```

**Critical Flow 2: Build Strategy**:
```typescript
// tests/e2e/strategy-builder.spec.ts
test('build bull call spread', async ({ page }) => {
  await page.goto('/app/options/strategy');
  
  // Select strategy template
  await page.click('[data-testid="strategy-bull-call-spread"]');
  
  // Configure legs
  await page.fill('[data-testid="long-call-strike"]', '150');
  await page.fill('[data-testid="short-call-strike"]', '155');
  
  // Verify payoff diagram
  await expect(page.locator('[data-testid="payoff-diagram"]')).toBeVisible();
  
  // Save strategy
  await page.click('[data-testid="save-strategy"]');
  await expect(page.locator('[data-testid="success-toast"]')).toBeVisible();
});
```

---

## Integration with AlphaForge Core

### Workspace Integration

Option entities follow the workspace-scoped pattern:

```rust
// All option entities have workspace_id
pub struct OptionChain {
    pub id: String,
    pub workspace_id: String,  // ← Links to workspace
    pub symbol: String,
    // ...
}

// Repository queries are workspace-scoped
impl OptionChainRepository {
    pub async fn find_by_workspace(
        &self,
        workspace_id: &str
    ) -> Result<Vec<OptionChain>, AppError> {
        sqlx::query_as!(
            OptionChain,
            r#"
                SELECT * FROM option_chains
                WHERE workspace_id = ?
                ORDER BY created_at DESC
            "#,
            workspace_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(Into::into)
    }
}
```

### Artifact Integration

Option analysis results create artifacts:

```rust
// Agent produces option analysis
pub async fn analyze_mispricing(
    symbol: &str,
    state: &AppState
) -> Result<Artifact, AppError> {
    // 1. Fetch chain
    let chain = state.option_service.fetch_chain(symbol).await?;
    
    // 2. Analyze for mispricing
    let analysis = analyze_iv_opportunities(&chain);
    
    // 3. Create artifact
    let artifact = Artifact {
        id: Uuid::new_v4().to_string(),
        workspace_id: chain.workspace_id,
        artifact_type: "option-chain".to_string(),
        input: serde_json::to_value(&chain)?,
        output: serde_json::to_value(&analysis)?,
        status: ArtifactStatus::Completed,
        created_at: Utc::now(),
    };
    
    state.artifact_repository.save(&artifact).await?;
    
    Ok(artifact)
}
```

### Portfolio Integration

Option positions integrate with portfolio tracking:

```rust
pub struct PortfolioRiskAnalysis {
    pub equity_positions: Vec<Position>,
    pub option_positions: Vec<OptionPosition>,
    pub net_greeks: PortfolioGreeks,
}

impl PortfolioService {
    pub async fn calculate_total_risk(
        &self,
        workspace_id: &str
    ) -> Result<PortfolioRiskAnalysis, AppError> {
        // Load equity and option positions
        let equities = self.position_repo.find_by_workspace(workspace_id).await?;
        let options = self.option_position_repo.find_by_workspace(workspace_id).await?;
        
        // Calculate combined risk
        let net_greeks = self.greeks_aggregator.aggregate(&options).await?;
        
        Ok(PortfolioRiskAnalysis {
            equity_positions: equities,
            option_positions: options,
            net_greeks,
        })
    }
}
```

---

## Deployment Considerations

### Build Configuration

```toml
# crates/option-core/Cargo.toml
[package]
name = "option-core"
version = "0.1.0"
edition = "2021"

[dependencies]
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
rayon = "1.8"  # Parallel calculation

[dev-dependencies]
criterion = "0.5"  # Performance benchmarking

[[bench]]
name = "pricing_benchmark"
harness = false
```

### Feature Flags

```toml
# apps/desktop/src-tauri/Cargo.toml
[features]
default = ["options"]
options = ["dep:option-core"]
options-live-provider = ["options", "dep:reqwest"]
options-backtesting = ["options"]
```

### Performance Monitoring

```rust
// Instrument pricing functions
#[instrument(skip_all, fields(symbol = %contract.symbol))]
pub fn calculate_greeks(contract: &OptionContract) -> Greeks {
    let start = std::time::Instant::now();
    
    let greeks = compute_greeks_analytical(contract);
    
    metrics::histogram!("option.greeks.calculation_time", start.elapsed());
    
    greeks
}
```

---

## Future Architecture Evolution

### Phase 7+ Enhancements (Optional)

**1. Real-Time Data Streaming**:
- WebSocket connection to market data provider
- Event-driven architecture for live updates
- Incremental Greeks recalculation

**2. Advanced Analytics**:
- Machine learning for IV prediction
- Monte Carlo simulation for exotic options
- Portfolio optimization algorithms

**3. Collaboration Features**:
- Shared workspaces with permissions
- Real-time collaboration on strategies
- Comment and annotation system

**4. Plugin Marketplace**:
- Third-party option analysis plugins
- Signed plugin verification
- Sandboxed plugin execution

---

## References

- [Option Documentation Index](./README.md)
- [Option Implementation Details](./IMPLEMENTATION_DETAILS.md)
- [Option Integration Plan](./INTEGRATION_PLAN.md)
- [Product Specification](./PRODUCT.md)
- [Roadmap](./ROADMAP.md)
- [Use Cases](./USE_CASES.md)
- [Data Model](./DATA_MODEL.md)
- [API Specification](./API_SPEC.md)
- [AlphaForge Architecture](../ARCHITECTURE.md)
- [AlphaForge Artifact System](../ARTIFACT_SYSTEM.md)
- [AlphaForge Plugin Spec](../PLUGIN_SPEC.md)
- [AlphaForge Security](../SECURITY.md)
