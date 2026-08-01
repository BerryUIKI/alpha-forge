/**
 * Empty Chain - Empty state component
 */

import { Search } from 'lucide-react';

interface EmptyChainProps {
  symbol: string;
}

export function EmptyChain({ symbol }: EmptyChainProps) {
  return (
    <div 
      className="flex flex-col items-center justify-center py-12 px-4"
      role="status"
      aria-live="polite"
    >
      <Search 
        className="h-12 w-12 text-gray-400 mb-4"
        aria-hidden="true"
      />
      
      <h3 className="text-lg font-semibold text-gray-900 mb-2">
        No Options Available
      </h3>
      
      <p className="text-gray-600 text-center max-w-md">
        No option contracts found for symbol <span className="font-medium">{symbol}</span>.
        This may be because the symbol is invalid, no options are listed, or data is temporarily unavailable.
      </p>
    </div>
  );
}

export default EmptyChain;