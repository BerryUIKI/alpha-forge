// Portfolio desktop API.

import { invoke } from "@tauri-apps/api/core";

export async function listPortfolioAccounts(): Promise<string[]> {
  return invoke("list_portfolio_accounts");
}
