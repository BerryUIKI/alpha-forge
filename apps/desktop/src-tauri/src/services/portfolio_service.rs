use crate::database::repositories::portfolio_repository::PortfolioRepository;
use crate::error::AppError;
use std::collections::{HashMap, HashSet};
use domain::portfolio::{
    CreatePortfolioAccountInput, CreatePortfolioTransactionInput, CreatePositionInput,
    ConcentrationRisk, ConcentrationSeverity, CreatePortfolioThemeLinkInput, PortfolioAccount, PortfolioAllocation, PortfolioReview,
    PortfolioTransaction, Position, TransactionType,
};
pub struct PortfolioService {
    repo: PortfolioRepository,
}
impl PortfolioService {
    pub fn new(repo: PortfolioRepository) -> Self {
        Self { repo }
    }
    pub async fn create_account(
        &self,
        input: CreatePortfolioAccountInput,
    ) -> Result<PortfolioAccount, AppError> {
        if input.name.trim().is_empty()
            || input.account_type.trim().is_empty()
            || input.currency.trim().is_empty()
        {
            return Err(AppError::Validation(
                "Account name, type, and currency are required".to_string(),
            ));
        }
        self.repo.create_account(input).await
    }
    pub async fn list_accounts(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<PortfolioAccount>, AppError> {
        self.repo.list_accounts(workspace_id).await
    }
    pub async fn create_position(&self, input: CreatePositionInput) -> Result<Position, AppError> {
        if input.symbol.trim().is_empty() || input.quantity == 0.0 {
            return Err(AppError::Validation(
                "Position symbol and non-zero quantity are required".to_string(),
            ));
        }
        if self.repo.get_account(&input.account_id).await?.is_none() {
            return Err(AppError::NotFound(
                "Portfolio account not found".to_string(),
            ));
        }
        self.repo.create_position(input).await
    }
    pub async fn list_positions(&self, account_id: &str) -> Result<Vec<Position>, AppError> {
        self.repo.list_positions(account_id).await
    }
    pub async fn import_transactions_csv(&self, account_id: &str, csv_text: &str) -> Result<Vec<PortfolioTransaction>, AppError> {
        if self.repo.get_account(account_id).await?.is_none() { return Err(AppError::NotFound("Portfolio account not found".to_string())); }
        let mut reader = csv::ReaderBuilder::new().trim(csv::Trim::All).from_reader(csv_text.as_bytes());
        let headers = reader.headers().map_err(|error| AppError::Validation(format!("Invalid CSV header: {error}")))?;
        let required = ["symbol", "transaction_type", "quantity", "price", "executed_at"];
        if headers.iter().map(|header| header.to_ascii_lowercase()).collect::<Vec<_>>() != required { return Err(AppError::Validation("CSV columns must be: symbol,transaction_type,quantity,price,executed_at".to_string())); }
        let mut inputs = Vec::new();
        for (index, record) in reader.records().enumerate() {
            let record = record.map_err(|error| AppError::Validation(format!("Invalid CSV row {}: {error}", index + 2)))?;
            let symbol = record.get(0).unwrap_or_default().trim().to_ascii_uppercase();
            let transaction_type = TransactionType::parse(record.get(1).unwrap_or_default()).ok_or_else(|| AppError::Validation(format!("Row {} must use buy or sell", index + 2)))?;
            let quantity = record.get(2).unwrap_or_default().parse::<f64>().map_err(|_| AppError::Validation(format!("Row {} has an invalid quantity", index + 2)))?;
            let price = record.get(3).unwrap_or_default().parse::<f64>().map_err(|_| AppError::Validation(format!("Row {} has an invalid price", index + 2)))?;
            let executed_at = chrono::DateTime::parse_from_rfc3339(record.get(4).unwrap_or_default()).map_err(|_| AppError::Validation(format!("Row {} must use an RFC 3339 execution timestamp", index + 2)))?.with_timezone(&chrono::Utc);
            if symbol.is_empty() || quantity <= 0.0 || !quantity.is_finite() || price < 0.0 || !price.is_finite() { return Err(AppError::Validation(format!("Row {} requires a symbol, positive finite quantity, and non-negative finite price", index + 2))); }
            inputs.push(CreatePortfolioTransactionInput { account_id: account_id.to_string(), symbol, transaction_type, quantity, price, executed_at });
        }
        if inputs.is_empty() { return Err(AppError::Validation("CSV must contain at least one transaction row".to_string())); }
        self.repo.import_transactions(inputs).await
    }
    pub async fn list_transactions(&self, account_id: &str) -> Result<Vec<PortfolioTransaction>, AppError> { self.repo.list_transactions(account_id).await }
    pub async fn allocation_by_workspace(&self, workspace_id: &str) -> Result<Vec<PortfolioAllocation>, AppError> {
        let positions = self.repo.list_positions_by_workspace(workspace_id).await?;
        let mut totals: HashMap<String, (f64, HashSet<String>)> = HashMap::new();
        for position in positions {
            let allocated_cost = position.cost_basis.unwrap_or(0.0) * position.quantity.abs();
            let entry = totals.entry(position.symbol).or_insert_with(|| (0.0, HashSet::new()));
            entry.0 += allocated_cost;
            entry.1.insert(position.account_id);
        }
        let total_cost: f64 = totals.values().map(|(cost, _)| cost).sum();
        let mut allocations: Vec<_> = totals.into_iter().map(|(symbol, (allocated_cost, accounts))| PortfolioAllocation { symbol, allocated_cost, weight_percent: if total_cost > 0.0 { allocated_cost / total_cost * 100.0 } else { 0.0 }, account_count: accounts.len() }).collect();
        allocations.sort_by(|left, right| right.allocated_cost.total_cmp(&left.allocated_cost));
        Ok(allocations)
    }
    pub async fn concentration_risks(&self, workspace_id: &str) -> Result<Vec<ConcentrationRisk>, AppError> {
        Ok(self.allocation_by_workspace(workspace_id).await?.into_iter().filter_map(|allocation| {
            let severity = if allocation.weight_percent >= 25.0 { Some(ConcentrationSeverity::High) } else if allocation.weight_percent >= 10.0 { Some(ConcentrationSeverity::Moderate) } else { None }?;
            let label = match severity { ConcentrationSeverity::High => "high", ConcentrationSeverity::Moderate => "moderate" };
            let message = format!("{} is a {label} cost-basis concentration at {:.1}% of recorded allocation.", allocation.symbol, allocation.weight_percent);
            Some(ConcentrationRisk { symbol: allocation.symbol, weight_percent: allocation.weight_percent, severity, message })
        }).collect())
    }
    pub async fn link_theme(&self, input: CreatePortfolioThemeLinkInput) -> Result<(), AppError> { if input.symbol.trim().is_empty() || input.entity_id.trim().is_empty() { return Err(AppError::Validation("A symbol and theme are required".to_string())); } self.repo.create_theme_link(CreatePortfolioThemeLinkInput { symbol: input.symbol.trim().to_ascii_uppercase(), ..input }).await }
    pub async fn theme_exposure(&self, workspace_id: &str) -> Result<Vec<domain::portfolio::ThemeExposure>, AppError> { self.repo.theme_exposure(workspace_id).await }
    pub async fn thesis_alignment(&self, workspace_id: &str) -> Result<Vec<domain::portfolio::ThesisAlignment>, AppError> { self.repo.thesis_alignment(workspace_id).await }
    pub async fn review(&self, workspace_id: &str) -> Result<PortfolioReview, AppError> { let allocations = self.allocation_by_workspace(workspace_id).await?; let aligned: HashSet<_> = self.repo.thesis_alignment(workspace_id).await?.into_iter().map(|item| item.symbol).collect(); Ok(PortfolioReview { generated_at: chrono::Utc::now(), concentration_risks: self.concentration_risks(workspace_id).await?, unaligned_symbols: allocations.into_iter().map(|item| item.symbol).filter(|symbol| !aligned.contains(symbol)).collect() }) }
}
