/**
 * Chain Error - Error state component
 */

import { AlertCircle, RefreshCw } from 'lucide-react';

interface ChainErrorProps {
  error: Error;
  onRetry: () => void;
}

export function ChainError({ error, onRetry }: ChainErrorProps) {
  return (
    <div 
      className="flex flex-col items-center justify-center py-12 px-4"
      role="alert"
      aria-live="assertive"
    >
      <AlertCircle 
        className="h-12 w-12 text-red-500 mb-4"
        aria-hidden="true"
      />
      
      <h3 className="text-lg font-semibold text-gray-900 mb-2">
        Failed to Load Option Chain
      </h3>
      
      <p className="text-gray-600 mb-4 text-center max-w-md">
        {error.message || 'An unexpected error occurred while fetching option data.'}
      </p>

      <button
        onClick={onRetry}
        className="inline-flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors"
        aria-label="Retry loading option chain"
      >
        <RefreshCw className="h-4 w-4" />
        Try Again
      </button>
    </div>
  );
}

export default ChainError;