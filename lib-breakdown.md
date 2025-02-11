# rs-quant lib breakdown

## 1. core data structure and handling
- [x] **time series data**: efficient storage and retrieval of historical price data.
- [ ] **tick data and order book**: structures for high-frequency trading (_HFT_).
- [ ] **market data APIs**: integration with real-time data sources:
  - [x] yahoo! finance.
  - [ ] financial modeling prep.
  - [ ] alpha vantage.
  - [ ] bloomberg.
  - [ ] factset.
  - [ ] refinitiv.
  - [ ] morningstar.
- [ ] **portfolio data**: holdings, wights, returns, and risk metrics.
- [ ] **custom data loader**: support for CSV, JSON, Parquet, database (MySql and postgres), nosql databases sources.

## 2. math foundation
- [ ] **linear algebra**: matrix operations, vector calculus (`ndarray` crate should be used).
- [ ] **numerical methods**: root finding (_Newton-Raphson_), optimization (_gradient descent_).
- [ ] **stochastic processes**:
  - [ ] brownian motion.
  - [ ] ornstein-uhlenbeck process (mean reversion).
  - [ ] lévy processes.
- [ ] **fourier and wavelet transforms** for signal processing.

## 3. statistical and machine learning
- [ ] **descriptive and inferential**:
  - [x] central tendency (mean, median).
  - [x] dispersion metrics (variance, standard deviation, skewness, kurtosis)
  - [ ] hypothesis testing (t-test, anova, chi-squared tests).
- [ ] **timeseries analysis**:
  - [ ] moving averages (sma, ema, wma, wema).
  - [ ] volatility models (garch, egarch, heston).
  - [ ] arima/sarima/arma for forecasting.
  - [ ] kalman filters (state-space models).
  - [ ] hidden markov model.
- [ ] **machine learning for finance**:
  - [ ] regression models (linear, ridge, lasso, logitic, ecc.).
  - [ ] random forest and gradient boosting (xgboost, lightgbm).
  - [ ] neural networks (lstm, transformers for financial forecasting).
  - [ ] reinforcement learning (deep q-learning for algotrading).
  - [ ] sentiment analysis.

## 4. risk analysis and portfolio optimization
- [ ] **risk metrics**:
  - [ ] var (value at risk): parametric, historical, monte carlo.
  - [ ] cvar (conditional var), expected shortfall.
  - [ ] drawdowns and max drawdown.
  - [ ] sharpe ratio, sortino, calmar.
  - [ ] kelly criterion for optimal bet sizing.
- [ ] **portfolio optimization**:
  - [ ] modern portfolio theory: mean-variance optmization (markowitz).
  - [ ] black-litterman model: bayesian asset allocation.
  - [ ] risk parity: equal risk contribution across asset.
  - [ ] factor model: fama-french, arbitrage pricing theory.
  - [ ] bayesian portfolio optimization: hierarchical risk parity.

## 5. derivatives pricing and greeks calculation
- [ ] **option pricing models**:
  - [ ] black-scholes-merton.
  - [ ] binomial tree (cox-ross-rubenstein).
  - [ ] monte carlo simulation for derivatives.
  - [ ] heston model (stochastic volatility).
- [ ] **greek calculation**:
  - [ ] delta, gamma, vega, theta, rho.
  - [ ] second-order greeks (vanna, charm, vomma).
- [ ] **interest rate model**:
  - [ ] hull-white model.
  - [ ] vasicek model.
  - [ ] cox-ingersoll-ross model.

## 6. algo trading and backtesting
- [ ] **backtesting framework**:
  - [ ] event-driven architecture (market data, execution, portfolio updates).
  - [ ] performance metrics (cagr, sharpe, drawdowns).
  - [ ] slippage and transaction cost modeling.
- [ ] **execution strategies**:
  - [ ] vwap, twap (for large orders).
  - [ ] smart order routing (sor).
  - [ ] market making strategies.
- [ ] **high-frequency trading**:
  - [ ] latency-sensitive order book modeling.
  - [ ] market microstructure analysis.
  - [ ] order imbalance strategies.

## 7. fixed income and credit risk models
- [ ] **yield curve constructing** (bootstrapping, nelson-siegel-svensson).
- [ ] **bond pricing and duration** (bond,pricing, duration, yields, convexity).
- [ ] **credit risk model**:
  - [ ] merton model (structural model).
  - [ ] credit metrics (default correlation).
  - [ ] credit default swap (cds) pricing.

## 8. alternative data and sentiment analysis
- [ ] **sentiment analysis** (news, social media, ecc.).
- [ ] **alternative datasets** (satellite images, credit card transactions, ecc.).
- [ ] **natural language processing** (nlp):
  - [ ] topic modeling (lda, bert).
  - [ ] named entity recognition (ner) for financial reports.

## 9. monte carlo simulation for risk adn pricing
- [ ] **asset pricing simulation** (gbm, jump-diffusion, ecc.).
- [ ] **monte carlo** for option pricing.
- [ ] **risk scenario generator**.
