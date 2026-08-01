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
};