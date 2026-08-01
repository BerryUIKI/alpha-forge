/**
 * Chain Skeleton - Loading state component
 */

export function ChainSkeleton() {
  return (
    <div 
      className="animate-pulse"
      role="status"
      aria-label="Loading option chain"
    >
      {/* Header skeleton */}
      <div className="mb-4">
        <div className="h-6 bg-gray-200 rounded w-48 mb-2" />
        <div className="h-4 bg-gray-200 rounded w-32" />
      </div>

      {/* Filters skeleton */}
      <div className="flex gap-4 mb-4">
        {[1, 2, 3, 4].map((i) => (
          <div key={i} className="space-y-1">
            <div className="h-4 bg-gray-200 rounded w-20" />
            <div className="h-10 bg-gray-200 rounded w-24" />
          </div>
        ))}
      </div>

      {/* Table skeleton */}
      <div className="border rounded">
        <div className="bg-gray-50 h-12" />
        {[1, 2, 3, 4, 5, 6, 7, 8].map((i) => (
          <div 
            key={i} 
            className="h-12 border-t flex items-center"
          >
            <div className="flex-1 flex">
              {[1, 2, 3, 4, 5].map((j) => (
                <div 
                  key={j}
                  className="flex-1 h-4 bg-gray-200 rounded mx-2"
                />
              ))}
            </div>
            <div className="w-20 h-4 bg-gray-200 rounded mx-2" />
            <div className="flex-1 flex">
              {[1, 2, 3, 4, 5].map((j) => (
                <div 
                  key={j}
                  className="flex-1 h-4 bg-gray-200 rounded mx-2"
                />
              ))}
            </div>
          </div>
        ))}
      </div>

      <span className="sr-only">Loading option chain data...</span>
    </div>
  );
}

export default ChainSkeleton;