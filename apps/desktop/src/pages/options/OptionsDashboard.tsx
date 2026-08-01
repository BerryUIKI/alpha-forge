/**
 * Options Dashboard Page
 */

import { useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import { optionsApi } from '@/lib/desktop-api/options';

export function OptionsDashboard() {
  const [symbol, setSymbol] = useState('AAPL');

  return (
    <div className="p-6">
      <h1 className="text-2xl font-bold mb-4">Options Analysis Platform</h1>
      
      <div className="space-y-4">
        <div>
          <label className="block text-sm font-medium mb-1">Symbol</label>
          <input 
            type="text" 
            value={symbol}
            onChange={(e) => setSymbol(e.target.value)}
            className="border rounded px-3 py-2 w-48"
            placeholder="AAPL"
          />
        </div>

        <p className="text-gray-600">
          Phase 1-2 complete. Frontend foundation in progress.
        </p>
      </div>
    </div>
  );
}

export default OptionsDashboard;