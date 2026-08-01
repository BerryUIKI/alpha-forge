/**
 * Greeks Form Component
 * Input form for Greeks calculation parameters
 */

import { useState } from 'react';
import type { CalculateGreeksParams } from '@/types/option';

export interface GreeksFormProps {
  onSubmit: (params: CalculateGreeksParams) => void;
  isLoading?: boolean;
}

export function GreeksForm({ onSubmit, isLoading }: GreeksFormProps) {
  const [formData, setFormData] = useState<CalculateGreeksParams>({
    optionType: 'call',
    underlyingPrice: 100,
    strike: 100,
    expirationYears: 0.25,
    riskFreeRate: 0.05,
    volatility: 0.20,
    dividendYield: 0,
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSubmit(formData);
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4 p-4 border rounded-lg">
      <div>
        <label className="block text-sm font-medium mb-1">Option Type</label>
        <select
          value={formData.optionType}
          onChange={(e) => setFormData({ ...formData, optionType: e.target.value as 'call' | 'put' })}
          className="w-full border rounded px-3 py-2"
          disabled={isLoading}
        >
          <option value="call">Call</option>
          <option value="put">Put</option>
        </select>
      </div>

      <div className="grid grid-cols-2 gap-4">
        <div>
          <label className="block text-sm font-medium mb-1">Underlying Price</label>
          <input
            type="number"
            step="0.01"
            value={formData.underlyingPrice}
            onChange={(e) => setFormData({ ...formData, underlyingPrice: parseFloat(e.target.value) })}
            className="w-full border rounded px-3 py-2"
            disabled={isLoading}
          />
        </div>

        <div>
          <label className="block text-sm font-medium mb-1">Strike Price</label>
          <input
            type="number"
            step="0.01"
            value={formData.strike}
            onChange={(e) => setFormData({ ...formData, strike: parseFloat(e.target.value) })}
            className="w-full border rounded px-3 py-2"
            disabled={isLoading}
          />
        </div>
      </div>

      <div className="grid grid-cols-2 gap-4">
        <div>
          <label className="block text-sm font-medium mb-1">Expiration (Years)</label>
          <input
            type="number"
            step="0.01"
            value={formData.expirationYears}
            onChange={(e) => setFormData({ ...formData, expirationYears: parseFloat(e.target.value) })}
            className="w-full border rounded px-3 py-2"
            disabled={isLoading}
          />
        </div>

        <div>
          <label className="block text-sm font-medium mb-1">Volatility (%)</label>
          <input
            type="number"
            step="0.01"
            value={formData.volatility * 100}
            onChange={(e) => setFormData({ ...formData, volatility: parseFloat(e.target.value) / 100 })}
            className="w-full border rounded px-3 py-2"
            disabled={isLoading}
          />
        </div>
      </div>

      <div className="grid grid-cols-2 gap-4">
        <div>
          <label className="block text-sm font-medium mb-1">Risk-Free Rate (%)</label>
          <input
            type="number"
            step="0.01"
            value={formData.riskFreeRate * 100}
            onChange={(e) => setFormData({ ...formData, riskFreeRate: parseFloat(e.target.value) / 100 })}
            className="w-full border rounded px-3 py-2"
            disabled={isLoading}
          />
        </div>

        <div>
          <label className="block text-sm font-medium mb-1">Dividend Yield (%)</label>
          <input
            type="number"
            step="0.01"
            value={(formData.dividendYield || 0) * 100}
            onChange={(e) => setFormData({ ...formData, dividendYield: parseFloat(e.target.value) / 100 })}
            className="w-full border rounded px-3 py-2"
            disabled={isLoading}
          />
        </div>
      </div>

      <button
        type="submit"
        disabled={isLoading}
        className="w-full bg-blue-600 text-white rounded px-4 py-2 hover:bg-blue-700 disabled:opacity-50"
      >
        {isLoading ? 'Calculating...' : 'Calculate Greeks'}
      </button>
    </form>
  );
}