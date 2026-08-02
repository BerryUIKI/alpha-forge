# Option Analysis Platform - Use Cases

## Overview

This document defines detailed user stories and workflows for the Option Analysis Platform. Each use case follows the format:

- **Actor**: Who is performing the action
- **Goal**: What they want to achieve
- **Preconditions**: What must be true before starting
- **Main Flow**: The primary sequence of steps
- **Alternative Flows**: Variations and error handling
- **Postconditions**: What is true after completion

---

## Use Case 1: Analyze Option Chain for Mispricing

### Actor
**Options Trader** looking for mispriced options opportunities

### Goal
Identify options with favorable implied volatility relative to historical patterns or theoretical values.

### Preconditions
- User has created or selected a workspace
- User has configured a data provider (demo or live)
- User knows the underlying symbol they want to analyze

### Main Flow

1. **Navigate to Option Chain**
   - User opens Options page from main navigation
   - System displays option chain interface with symbol search

2. **Select Underlying**
   - User enters symbol (e.g., "AAPL") in search field
   - System validates symbol exists
   - System fetches current option chain data from provider
   - System displays loading state while fetching

3. **View Chain Data**
   - System renders option chain table with:
     - Calls on left, puts on right
     - Strikes sorted by moneyness
     - Bid/ask spread, volume, open interest
     - Implied volatility for each option
   - User can see current underlying price

4. **Analyze Implied Volatility**
   - User clicks "IV Analysis" button
   - System calculates IV percentile and rank
   - System compares current IV to 30-day historical IV
   - System highlights options with IV significantly above/below average

5. **Filter for Opportunities**
   - User applies filters:
     - Expiration range (e.g., 30-60 days)
     - Minimum volume (e.g., > 100)
     - IV percentile threshold (e.g., > 70% or < 30%)
   - System updates chain display with filtered results

6. **Drill Down into Option**
   - User clicks on specific option row
   - System displays detailed view with:
     - Full Greeks display (Delta, Gamma, Theta, Vega, Rho)
     - Historical IV chart
     - Volume and open interest trends
     - Bid/ask spread analysis

7. **Compare to Theoretical Value**
   - User clicks "Calculate Theoretical" button
   - User enters volatility assumption (e.g., historical realized vol)
   - System calculates theoretical price using Black-Scholes
   - System displays difference between market price and theoretical

8. **Save Analysis**
   - User identifies mispriced option
   - User clicks "Save to Workspace" button
   - System saves analysis artifact to current workspace
   - User can add notes about the opportunity

### Alternative Flows

**3a. Symbol Not Found**
- System displays error: "Symbol not found"
- User corrects symbol or selects from suggestions

**3b. No Options Available**
- System displays empty state: "No options available for this symbol"
- User selects different symbol

**4a. Insufficient Historical Data**
- System shows limited IV analysis (no percentile/rank)
- User proceeds with available data

### Postconditions
- Option chain displayed with IV analysis
- Mispricing opportunities identified
- Analysis saved to workspace for future reference

### Acceptance Criteria
- [ ] Chain loads within 2 seconds
- [ ] IV calculations accurate (tested against known values)
- [ ] Filtering works correctly
- [ ] Greeks displayed for selected options
- [ ] Analysis can be saved and retrieved later

---

## Use Case 2: Build and Analyze Bull Call Spread

### Actor
**Individual Investor** wanting to construct a bullish strategy with limited risk

### Goal
Create and analyze a bull call spread strategy to understand risk/reward profile.

### Preconditions
- User has analyzed option chain (Use Case 1)
- User has identified target expiration and strike range
- User wants to limit downside risk while capturing upside

### Main Flow

1. **Open Strategy Builder**
   - User clicks "Build Strategy" button from chain view
   - System opens strategy builder interface
   - System displays strategy templates

2. **Select Strategy Template**
   - User selects "Bull Call Spread" from template list
   - System pre-populates strategy with:
     - Two legs (long call, short call)
     - Same expiration
     - Different strikes
   - System shows strategy configuration panel

3. **Configure Long Call Leg**
   - User selects lower strike for long call (e.g., $150)
   - System fetches bid/ask for selected option
   - User enters quantity (e.g., 1 contract = 100 shares)
   - System calculates cost (premium to pay)

4. **Configure Short Call Leg**
   - User selects higher strike for short call (e.g., $155)
   - System fetches bid/ask for selected option
   - User enters quantity (e.g., 1 contract)
   - System calculates credit received

5. **View Strategy Summary**
   - System calculates net debit/credit
   - System displays:
     - Net cost: $2.50 per share ($250 total)
     - Max profit: $250 (if stock at or above $155)
     - Max loss: $250 (if stock at or below $150)
     - Break-even: $152.50

6. **Analyze Payoff Diagram**
   - System renders interactive payoff diagram
   - Diagram shows:
     - X-axis: Underlying price at expiration
     - Y-axis: Profit/Loss
     - Strategy P&L line
     - Break-even point highlighted
     - Max profit/loss zones shaded
   - User can hover over chart to see P&L at specific prices

7. **View Greeks**
   - System calculates net Greeks for combined position
   - System displays:
     - Net Delta: Positive (bullish bias)
     - Net Gamma: Near zero (limited gamma exposure)
     - Net Theta: Positive (time decay helps)
     - Net Vega: Near zero (low volatility sensitivity)

8. **Run Scenario Analysis**
   - User clicks "Scenario Analysis" button
   - User defines scenarios:
     - Bullish: Stock up 5%
     - Neutral: Stock unchanged
     - Bearish: Stock down 5%
   - System calculates P&L for each scenario
   - User sees expected outcomes

9. **Save Strategy**
   - User clicks "Save Strategy" button
   - User enters strategy name: "AAPL Bull Call Spread Jan19"
   - System saves strategy to workspace
   - Strategy appears in "Saved Strategies" list

### Alternative Flows

**5a. Invalid Strike Combination**
- System shows warning: "Short call strike must be higher than long call strike"
- User adjusts strike selection

**5b. Insufficient Liquidity**
- System shows warning: "Low liquidity on selected options"
- User can proceed or choose different strikes

**9a. User Wants to Track Position**
- User clicks "Add to Portfolio" instead
- System creates option positions in portfolio
- User can track P&L over time

### Postconditions
- Bull call spread strategy created
- Risk/reward profile understood
- Strategy saved for future reference

### Acceptance Criteria
- [ ] Template selection works correctly
- [ ] Strike selection updates payoff diagram in real-time
- [ ] Payoff calculations accurate
- [ ] Break-even point correct
- [ ] Net Greeks calculated correctly
- [ ] Scenario analysis produces correct results
- [ ] Strategy persists to workspace

---

## Use Case 3: Analyze Portfolio Option Risk

### Actor
**Portfolio Manager** managing multiple option positions across different underlyings

### Goal
Understand portfolio-level risk exposure and identify concentration risks.

### Preconditions
- User has option positions in portfolio (imported or manually entered)
- Positions have associated Greeks calculated
- User wants to understand net exposure

### Main Flow

1. **Open Portfolio Risk Dashboard**
   - User navigates to "Portfolio" → "Options Risk"
   - System loads all option positions from database
   - System fetches current market data for each position
   - System calculates current Greeks for each position

2. **View Portfolio Summary**
   - System displays portfolio-level metrics:
     - Total positions: 15
     - Underlyings: 5 (AAPL, MSFT, GOOGL, TSLA, SPY)
     - Notional value: $150,000
     - Delta equivalent: $75,000 long

3. **Analyze Net Greeks**
   - System displays aggregated Greeks:
     - **Net Delta**: +$75,000 (bullish bias)
     - **Net Gamma**: +$2,500 (delta will increase if market rises)
     - **Net Theta**: -$500/day (time decay cost)
     - **Net Vega**: +$15,000 (long volatility exposure)
     - **Net Rho**: +$1,000 (sensitive to rates)

4. **View Greeks by Underlying**
   - System breaks down Greeks by symbol:
     - AAPL: Delta +$30,000
     - MSFT: Delta +$20,000
     - GOOGL: Delta +$15,000
     - TSLA: Delta +$10,000
     - SPY: Delta $0 (hedged)
   - User identifies concentration in AAPL

5. **Analyze Risk Contributions**
   - System shows risk contribution chart:
     - Pie chart of delta contribution by underlying
     - Bar chart of theta contribution by expiration
   - User sees that 40% of delta comes from AAPL

6. **Identify Concentration Risk**
   - System highlights concentration risks:
     - "High AAPL concentration: 40% of delta"
     - "Short-term theta risk: 80% of theta in < 30 days"
   - User recognizes need to hedge AAPL exposure

7. **Calculate Hedge Ratio**
   - User clicks "Calculate Hedge" button
   - System suggests:
     - "Sell 200 shares of AAPL to neutralize delta"
     - Or "Buy 2 AAPL $150 puts to hedge"
   - User can adjust hedge parameters

8. **Run Stress Test**
   - User clicks "Stress Test" button
   - User selects stress scenarios:
     - Market crash: -10% S&P 500
     - Volatility spike: +10% VIX
     - Interest rate hike: +1%
   - System calculates portfolio P&L under each scenario
   - System displays:
     - Market crash: -$15,000 loss
     - Volatility spike: +$5,000 gain
     - Rate hike: +$500 gain

9. **Export Risk Report**
   - User clicks "Export Report" button
   - System generates PDF report with:
     - Portfolio summary
     - Greeks breakdown
     - Risk contribution charts
     - Stress test results
   - User saves report for compliance/records

### Alternative Flows

**2a. No Positions**
- System shows empty state: "No option positions in portfolio"
- User is prompted to import or add positions

**3a. Greeks Data Stale**
- System shows warning: "Market data is 15 minutes old"
- User can refresh data

**7a. User Adjusts Portfolio**
- User decides to hedge by selling shares
- User clicks "Record Trade" button
- System updates portfolio with new position
- System recalculates Greeks

### Postconditions
- Portfolio risk understood
- Concentration risks identified
- Hedge ratios calculated
- Risk report generated

### Acceptance Criteria
- [ ] Portfolio loads within 3 seconds
- [ ] Greeks aggregation accurate
- [ ] Risk contributions calculated correctly
- [ ] Concentration risks identified
- [ ] Hedge suggestions reasonable
- [ ] Stress tests produce correct results
- [ ] Report generates successfully

---

## Use Case 4: Analyze Volatility Surface

### Actor
**Risk Analyst** examining implied volatility patterns across strikes and expirations

### Goal
Understand volatility surface structure to identify anomalies and inform strategy selection.

### Preconditions
- User has loaded option chain data
- Multiple expirations available
- User wants to visualize IV patterns

### Main Flow

1. **Open Volatility Analysis**
   - User clicks "Volatility Analysis" button
   - System retrieves IV data for all strikes and expirations
   - System prepares data for 3D surface rendering

2. **View 3D Volatility Surface**
   - System renders 3D surface plot:
     - X-axis: Moneyness (strike / underlying price)
     - Y-axis: Days to expiration
     - Z-axis: Implied volatility (%)
     - Color gradient: Low IV (blue) to High IV (red)
   - User can rotate, zoom, and pan the surface

3. **Identify Volatility Smile/Skew**
   - User observes that:
     - OTM puts have higher IV (left side elevated)
     - ATM options have lowest IV (center depressed)
     - OTM calls have slightly higher IV (right side elevated)
   - System labels this as "Volatility Smile" pattern

4. **Analyze Term Structure**
   - User switches to "Term Structure" view
   - System displays line chart:
     - X-axis: Days to expiration
     - Y-axis: ATM implied volatility
   - User sees IV term structure:
     - Short-term (30 days): 25%
     - Medium-term (60 days): 22%
     - Long-term (90 days): 20%
   - System notes: "Volatility in contango" (short-term > long-term)

5. **Compare Historical Surfaces**
   - User clicks "Compare Historical" button
   - User selects dates: Today, 1 week ago, 1 month ago
   - System overlays multiple surfaces
   - User sees how IV surface has changed over time

6. **Detect Anomalies**
   - System runs anomaly detection algorithm
   - System highlights unusual IV values:
     - "Unusually high IV for 60-day $160 calls"
     - "Unusually low IV for 30-day $140 puts"
   - User can investigate these options

7. **Export Surface Data**
   - User clicks "Export Data" button
   - System downloads CSV with:
     - All strikes, expirations, IV values
     - Timestamp
     - Underlying price
   - User can use data for further analysis

### Alternative Flows

**2a. Insufficient Data**
- System shows message: "Need at least 3 expirations for surface"
- User works with available expirations

**4a. Inverted Term Structure**
- System identifies: "Volatility in backwardation"
- Indicates market stress or event risk
- User notes elevated short-term IV

### Postconditions
- Volatility surface visualized
- Patterns and anomalies identified
- Data exported for analysis

### Acceptance Criteria
- [ ] 3D surface renders in < 2 seconds
- [ ] Interactive controls work smoothly (rotation, zoom)
- [ ] Term structure chart accurate
- [ ] Anomaly detection identifies outliers
- [ ] Data export works correctly

---

## Use Case 5: Backtest Covered Call Strategy

### Actor
**Individual Investor** evaluating historical performance of a covered call strategy

### Goal
Understand how a covered call strategy would have performed over a historical period.

### Preconditions
- User owns or is considering owning shares of a stock
- User wants to generate income by selling calls
- User has historical data available

### Main Flow

1. **Open Backtesting Tool**
   - User navigates to "Backtesting" page
   - System displays backtesting configuration form

2. **Define Strategy Parameters**
   - User enters:
     - Underlying: "AAPL"
     - Strategy: "Covered Call"
     - Position: Long 100 shares
     - Call strike selection: "ATM" or "5% OTM"
     - Expiration cycle: "Monthly"
     - Start date: "2023-01-01"
     - End date: "2023-12-31"

3. **Configure Assumptions**
   - User sets assumptions:
     - Initial capital: $15,000
     - Commission: $0.65 per contract
     - Slippage: $0.02 per share
     - Assignment: "If ITM at expiration"
     - Roll strategy: "Roll to next month if ITM"

4. **Run Backtest**
   - User clicks "Run Backtest" button
   - System loads historical data
   - System simulates month-by-month:
     - Buy 100 shares at start
     - Sell 1 ATM call each month
     - Track P&L at expiration
     - Roll or get assigned
   - System displays progress indicator

5. **View Performance Metrics**
   - System displays results:
     - **Total Return**: +12.5%
     - **Outperformance vs Buy-Hold**: +3.2%
     - **Sharpe Ratio**: 1.2
     - **Max Drawdown**: -8.5%
     - **Win Rate**: 75% of months profitable
     - **Total Premium Collected**: $2,400

6. **Analyze Month-by-Month Results**
   - User views detailed month table:
     - January: +$180 (stock up, call expired OTM)
     - February: -$50 (stock up, assigned, rolled)
     - March: +$220 (stock down, call expired OTM)
     - ... (12 months)
   - User can see premium collected each month

7. **Compare Scenarios**
   - User clicks "Compare Scenarios" button
   - User defines alternative strategies:
     - Scenario A: 5% OTM calls (less premium, more upside)
     - Scenario B: ATM calls (more premium, capped upside)
   - System runs both scenarios
   - System displays comparison chart

8. **View Greeks Evolution**
   - User clicks "Greeks Over Time" tab
   - System shows how delta changed month-to-month
   - User understands risk profile over time

9. **Save Backtest Results**
   - User clicks "Save Results" button
   - System saves backtest artifact to workspace
   - User can reference later for strategy validation

### Alternative Flows

**4a. Insufficient Historical Data**
- System shows error: "Data only available from 2023-06-01"
- User adjusts date range

**5a. Poor Performance**
- System shows negative results
- User experiments with different parameters
- User tries different strike selection (more OTM)

**9a. Export Results**
- User clicks "Export to CSV" instead
- System downloads detailed month-by-month data
- User can analyze in external spreadsheet

### Postconditions
- Covered call strategy backtested
- Performance metrics understood
- Results saved for reference

### Acceptance Criteria
- [ ] Backtest runs in < 30 seconds for 1-year period
- [ ] Performance calculations accurate
- [ ] Month-by-month breakdown correct
- [ ] Scenario comparison works
- [ ] Results can be saved and retrieved

---

## Use Case 6: Import Option Positions from Brokerage

### Actor
**Options Trader** who has existing positions at a brokerage and wants to track them

### Goal
Import existing option positions into AlphaForge for analysis and tracking.

### Preconditions
- User has access to brokerage account data
- User can export positions as CSV or manually enter
- User has created a portfolio account in workspace

### Main Flow

1. **Navigate to Portfolio Import**
   - User opens "Portfolio" page
   - User clicks "Import Positions" button
   - System displays import options:
     - CSV Upload
     - Manual Entry
     - API Connection (future feature)

2. **Select Import Method**
   - User selects "CSV Upload"
   - System displays file upload interface

3. **Upload CSV File**
   - User clicks "Choose File" button
   - User selects CSV file from computer
   - System validates file format:
     ```
     Symbol,Type,Strike,Expiration,Quantity,Multiplier,CostBasis
     AAPL,Call,150,2024-01-19,2,100,500.00
     MSFT,Put,350,2024-02-16,-1,100,-150.00
     ```
   - System parses CSV rows

4. **Validate Data**
   - System validates each row:
     - Symbol exists
     - Type is "Call" or "Put"
     - Strike is positive number
     - Expiration is valid date
     - Quantity is non-zero integer
   - System displays validation summary:
     - Valid positions: 15
     - Invalid positions: 2 (highlighted with errors)

5. **Review and Confirm**
   - User reviews parsed positions
   - User corrects invalid rows:
     - Fixes typo in symbol
     - Adjusts expiration date format
   - User clicks "Confirm Import" button

6. **Save Positions**
   - System creates `OptionPosition` records in database
   - System links positions to selected portfolio account
   - System calculates current Greeks for each position
   - System displays success message: "15 positions imported"

7. **View Updated Portfolio**
   - System navigates to portfolio view
   - User sees all imported positions
   - User can analyze portfolio-level risk (Use Case 3)

### Alternative Flows

**3a. Manual Entry Instead**
- User selects "Manual Entry" option
- User fills form for each position:
  - Symbol, Type, Strike, Expiration, Quantity, Cost Basis
- User clicks "Add Position" for each
- System saves individually

**4a. Validation Errors**
- System identifies specific errors:
  - "Invalid symbol: AAPLL (typo)"
  - "Expiration date in past: 2023-01-01"
- User must correct before proceeding

**6a. Duplicate Positions**
- System detects duplicates: "Position already exists"
- User can choose to:
  - Skip duplicate
  - Update existing
  - Add as new

### Postconditions
- Option positions imported successfully
- Positions linked to portfolio account
- Greeks calculated for each position
- Portfolio risk analysis available

### Acceptance Criteria
- [ ] CSV parsing handles standard formats
- [ ] Validation catches common errors
- [ ] Positions saved to database correctly
- [ ] Manual entry works smoothly
- [ ] Duplicate detection functional

---

## Use Case 7: Analyze Earnings Event Risk

### Actor
**Options Trader** holding positions through an earnings announcement

### Goal
Understand potential P&L scenarios around an earnings event.

### Preconditions
- User has option positions in a stock with upcoming earnings
- Earnings date is known
- User wants to understand event risk

### Main Flow

1. **Identify Earnings Event**
   - User opens portfolio view
   - System highlights positions with upcoming events:
     - "AAPL earnings in 5 days"
   - User clicks on alert

2. **Open Event Analysis**
   - System displays earnings event analysis:
     - Event date: 2024-01-25 after market close
     - Current positions: Long 5 AAPL $150 calls
     - Current P&L: +$200

3. **View Historical Earnings Moves**
   - System retrieves historical earnings moves:
     - Last 8 quarters average move: ±4.5%
     - Largest up move: +8.2%
     - Largest down move: -6.1%
   - User sees distribution of moves

4. **Run Scenario Analysis**
   - System runs scenarios based on historical data:
     - **Scenario 1** (Historical avg up): Stock +4.5%, P&L = +$1,200
     - **Scenario 2** (Historical avg down): Stock -4.5%, P&L = -$600
     - **Scenario 3** (Large up): Stock +8.2%, P&L = +$2,500
     - **Scenario 4** (Large down): Stock -6.1%, P&L = -$900
   - System displays probability-weighted expected value

5. **Analyze IV Crush**
   - System estimates post-earnings IV change:
     - Current IV: 45%
     - Expected post-earnings IV: 25%
     - Expected IV crush: -20 percentage points
   - System calculates vega impact: -$500 loss from IV crush

6. **Review Break-Even Points**
   - System calculates break-even stock moves:
     - Need stock to move at least +3.5% to profit
     - Current position profitable if stock > $153.50

7. **Evaluate Hedging Options**
   - System suggests hedging strategies:
     - "Buy protective puts to limit downside"
     - "Sell calls to finance put purchase (strangle)"
   - User can simulate adding hedge

8. **Make Decision**
   - Based on analysis, user decides:
     - Close position before earnings (avoid event risk)
     - Add hedge (limit downside)
     - Hold through earnings (accept event risk)

### Alternative Flows

**4a. No Historical Data**
- System shows limited analysis (no historical averages)
- User relies on implied move from straddle prices

**7a. User Simulates Hedge**
- User adds hedge to scenario analysis
- System recalculates P&L for all scenarios
- User sees improved risk profile

### Postconditions
- Earnings event risk understood
- Scenarios analyzed
- Hedging decision made

### Acceptance Criteria
- [ ] Earnings dates detected automatically
- [ ] Historical moves calculated correctly
- [ ] Scenario analysis accurate
- [ ] IV crush impact calculated
- [ ] Hedging suggestions reasonable

---

## Use Case 8: Educational - Learn About Greeks

### Actor
**Student** learning options theory and wanting interactive demonstrations

### Goal
Understand how option Greeks work and how they change with different parameters.

### Preconditions
- User is in demo mode (no real money)
- User has basic understanding of options
- User wants interactive learning experience

### Main Flow

1. **Open Greeks Tutorial**
   - User navigates to "Learn" section
   - User selects "Understanding Greeks" tutorial
   - System displays interactive lesson

2. **Learn Delta**
   - System shows: "Delta measures directional exposure"
   - Interactive demo:
     - User adjusts underlying price slider
     - System shows how Delta changes
     - Chart displays: Delta vs. Stock Price
   - User experiments with different options (ITM, ATM, OTM)
   - System explains: "Delta ≈ probability of finishing ITM"

3. **Learn Gamma**
   - System shows: "Gamma measures Delta's rate of change"
   - Interactive demo:
     - User sees Delta chart with Gamma overlay
     - User observes: Gamma highest for ATM options
     - User adjusts time to expiration
     - System shows: Gamma increases as expiration approaches

4. **Learn Theta**
   - System shows: "Theta measures time decay"
   - Interactive demo:
     - User adjusts days to expiration slider
     - System shows how option price decreases over time
     - Chart displays: Price vs. Days Remaining
   - User observes: Theta accelerates near expiration

5. **Learn Vega**
   - System shows: "Vega measures sensitivity to volatility"
   - Interactive demo:
     - User adjusts implied volatility slider
     - System shows how option price changes
     - User observes: Long options have positive Vega (benefit from IV increase)

6. **Learn Rho**
   - System shows: "Rho measures sensitivity to interest rates"
   - Interactive demo:
     - User adjusts risk-free rate slider
     - System shows price change
     - User notes: Rho usually small impact for short-dated options

7. **Interactive Quiz**
   - System presents quiz questions:
     - "Which Greek increases as expiration approaches?"
     - "What happens to Delta when stock price rises?"
   - User answers questions
   - System provides immediate feedback

8. **Practice Scenario**
   - User selects "Practice Mode"
   - System presents scenario: "You own an ATM call with 30 days to expiration. What happens to your position if..."
   - User answers multiple-choice questions about Greeks behavior
   - System provides explanations

### Alternative Flows

**3a. User Wants More Detail**
- User clicks "Deep Dive" button
- System shows mathematical derivation of Gamma
- User can see Black-Scholes partial derivatives

**7a. User Skips Quiz**
- User can skip quiz and proceed
- System tracks progress but marks quiz incomplete

### Postconditions
- User understands all five Greeks
- User can predict how Greeks change
- User ready to apply knowledge to real analysis

### Acceptance Criteria
- [ ] All Greeks explained clearly
- [ ] Interactive demos work smoothly
- [ ] Sliders update charts in real-time
- [ ] Quiz questions test understanding
- [ ] Progress tracked in workspace

---

## Summary

These use cases cover the primary workflows for the Option Analysis Platform:

1. **Chain Analysis** - Finding opportunities
2. **Strategy Building** - Constructing trades
3. **Portfolio Risk** - Managing exposure
4. **Volatility Analysis** - Understanding market structure
5. **Backtesting** - Validating strategies
6. **Position Management** - Tracking holdings
7. **Event Risk** - Handling special situations
8. **Education** - Learning concepts

Each use case maps to specific features and acceptance criteria, ensuring the platform meets real user needs.

## References

- [Option Documentation Index](./README.md)
- [Option Implementation Details](./IMPLEMENTATION_DETAILS.md)
- [Option Integration Plan](./INTEGRATION_PLAN.md)
- [Product Specification](./PRODUCT.md)
- [Architecture Design](./ARCHITECTURE.md)
- [API Specification](./API_SPEC.md)
