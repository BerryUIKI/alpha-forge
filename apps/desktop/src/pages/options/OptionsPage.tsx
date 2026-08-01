/**
 * Options Page - Main options analysis page
 */

import { useState } from 'react';
import { OptionChainViewer } from '@/features/options/OptionChainViewer';
import { Command, CommandPalette } from '@/components/CommandPalette';
import { Settings, RefreshCw } from 'lucide-react';

export function OptionsPage() {
  const [symbol, setSymbol] = useState('AAPL');
  const [showCommandPalette, setShowCommandPalette] = useState(false);

  // TODO: Get workspace from context
  const workspaceId = 'default-workspace';

  return (
    <div className="flex flex-col h-full">
      {/* Header */}
      <header className="border-b bg-white px-6 py-4">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-2xl font-bold text-gray-900">
              Options Analysis
            </h1>
            <p className="text-sm text-gray-500 mt-1">
              Professional option chain analysis and strategy building
            </p>
          </div>

          <div className="flex items-center gap-2">
            {/* Symbol Selector */}
            <div className="relative">
              <input
                type="text"
                value={symbol}
                onChange={(e) => setSymbol(e.target.value.toUpperCase())}
                placeholder="Symbol"
                className="w-32 px-3 py-2 border rounded-md text-sm font-medium uppercase"
                aria-label="Stock symbol"
              />
            </div>

            {/* Actions */}
            <button
              onClick={() => {/* TODO: Refresh */}}
              className="p-2 text-gray-600 hover:text-gray-800 rounded-md hover:bg-gray-100"
              aria-label="Refresh data"
            >
              <RefreshCw className="h-5 w-5" />
            </button>

            <button
              onClick={() => setShowCommandPalette(true)}
              className="p-2 text-gray-600 hover:text-gray-800 rounded-md hover:bg-gray-100"
              aria-label="Open command palette"
            >
              <Settings className="h-5 w-5" />
            </button>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="flex-1 overflow-auto p-6">
        <OptionChainViewer 
          symbol={symbol}
          workspaceId={workspaceId}
        />
      </main>

      {/* Command Palette */}
      {showCommandPalette && (
        <CommandPalette onClose={() => setShowCommandPalette(false)} />
      )}
    </div>
  );
}

export default OptionsPage;