/**
 * Chain Filters - Filter controls for option chain
 */

import { useState } from 'react';
import type { OptionType } from '@/types/option';

export interface FilterState {
  expirationRange: 'all' | 'near' | 'next' | 'far';
  strikeRange: 'all' | 'itm' | 'atm' | 'otm';
  minVolume: number;
  minOpenInterest: number;
}

interface ChainFiltersProps {
  filters: FilterState;
  onChange: (filters: FilterState) => void;
}

export function ChainFilters({ filters, onChange }: ChainFiltersProps) {
  const updateFilter = <K extends keyof FilterState>(
    key: K,
    value: FilterState[K]
  ) => {
    onChange({ ...filters, [key]: value });
  };

  return (
    <div 
      className="flex flex-wrap gap-4 items-end"
      role="group"
      aria-label="Option chain filters"
    >
      {/* Expiration Range */}
      <div>
        <label 
          htmlFor="expiration-filter"
          className="block text-sm font-medium text-gray-700 mb-1"
        >
          Expiration
        </label>
        <select
          id="expiration-filter"
          value={filters.expirationRange}
          onChange={(e) => updateFilter('expirationRange', e.target.value as FilterState['expirationRange'])}
          className="block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
        >
          <option value="all">All Dates</option>
          <option value="near">Near Term (&lt;30 days)</option>
          <option value="next">Next (30-60 days)</option>
          <option value="far">Far Term (&gt;60 days)</option>
        </select>
      </div>

      {/* Strike Range */}
      <div>
        <label 
          htmlFor="strike-filter"
          className="block text-sm font-medium text-gray-700 mb-1"
        >
          Strike
        </label>
        <select
          id="strike-filter"
          value={filters.strikeRange}
          onChange={(e) => updateFilter('strikeRange', e.target.value as FilterState['strikeRange'])}
          className="block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
        >
          <option value="all">All Strikes</option>
          <option value="itm">ITM</option>
          <option value="atm">ATM</option>
          <option value="otm">OTM</option>
        </select>
      </div>

      {/* Min Volume */}
      <div>
        <label 
          htmlFor="min-volume"
          className="block text-sm font-medium text-gray-700 mb-1"
        >
          Min Volume
        </label>
        <input
          id="min-volume"
          type="number"
          min={0}
          value={filters.minVolume}
          onChange={(e) => updateFilter('minVolume', parseInt(e.target.value) || 0)}
          className="block w-24 rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
          placeholder="0"
        />
      </div>

      {/* Min Open Interest */}
      <div>
        <label 
          htmlFor="min-oi"
          className="block text-sm font-medium text-gray-700 mb-1"
        >
          Min Open Interest
        </label>
        <input
          id="min-oi"
          type="number"
          min={0}
          value={filters.minOpenInterest}
          onChange={(e) => updateFilter('minOpenInterest', parseInt(e.target.value) || 0)}
          className="block w-24 rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
          placeholder="0"
        />
      </div>

      {/* Clear Filters */}
      <button
        onClick={() => onChange({
          expirationRange: 'all',
          strikeRange: 'all',
          minVolume: 0,
          minOpenInterest: 0,
        })}
        className="px-3 py-2 text-sm font-medium text-gray-600 hover:text-gray-800"
        aria-label="Clear all filters"
      >
        Clear Filters
      </button>
    </div>
  );
}

export default ChainFilters;