/**
 * Options API client for desktop IPC
 */

import { invoke } from '@tauri-apps/api/core';
import type { 
  OptionChain, 
  GreeksResponse, 
  CalculateGreeksParams,
  FetchChainParams 
} from '@/types/option';

export const optionsApi = {
  async fetchOptionChain(params: FetchChainParams): Promise<OptionChain> {
    return invoke<OptionChain>('fetch_option_chain', { params });
  },

  async calculateGreeks(params: CalculateGreeksParams): Promise<GreeksResponse> {
    return invoke<GreeksResponse>('calculate_greeks', { params });
  },

  async calculateOptionPrice(params: CalculateGreeksParams): Promise<number> {
    return invoke<number>('calculate_option_price', { params });
  },

  async calculatePortfolioGreeks(params: { workspaceId: string }): Promise<{
    net_delta: number;
    net_gamma: number;
    net_theta: number;
    net_vega: number;
    net_rho: number;
    delta_dollars: number;
    gamma_dollars: number;
    theta_dollars: number;
    vega_dollars: number;
  }> {
    return invoke('calculate_portfolio_greeks', { params });
  },
};