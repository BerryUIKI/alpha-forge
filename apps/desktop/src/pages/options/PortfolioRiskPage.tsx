/**
 * Portfolio Risk Page
 * Displays portfolio-level Greeks and risk analysis
 */

import { useQuery } from '@tanstack/react-query';
import { optionsApi } from '@/lib/desktop-api/options';

export function PortfolioRiskPage() {
  const { data: portfolioGreeks, isLoading } = useQuery({
    queryKey: ['portfolioGreeks'],
    queryFn: () => optionsApi.calculatePortfolioGreeks({
      workspaceId: 'default',
    }),
  });

  if (isLoading) {
    return (
      <div className="p-6">
        <div className="animate-pulse space-y-4">
          <div className="h-8 bg-gray-200 rounded w-1/4" />
          <div className="h-32 bg-gray-200 rounded" />
        </div>
      </div>
    );
  }

  return (
    <div className="p-6">
      <h1 className="text-2xl font-bold mb-6">Portfolio Risk Analysis</h1>

      {/* Greeks Summary */}
      <div className="grid grid-cols-5 gap-4 mb-6">
        <div className="p-4 border rounded-lg">
          <div className="text-sm text-gray-600">Net Delta</div>
          <div className="text-2xl font-bold">
            {portfolioGreeks?.net_delta?.toFixed(2) || '0.00'}
          </div>
          <div className="text-xs text-gray-500">
            ${portfolioGreeks?.delta_dollars?.toFixed(0) || '0'} exposure
          </div>
        </div>

        <div className="p-4 border rounded-lg">
          <div className="text-sm text-gray-600">Net Gamma</div>
          <div className="text-2xl font-bold">
            {portfolioGreeks?.net_gamma?.toFixed(4) || '0.0000'}
          </div>
          <div className="text-xs text-gray-500">
            ${portfolioGreeks?.gamma_dollars?.toFixed(0) || '0'} exposure
          </div>
        </div>

        <div className="p-4 border rounded-lg">
          <div className="text-sm text-gray-600">Net Theta</div>
          <div className="text-2xl font-bold text-red-600">
            {portfolioGreeks?.net_theta?.toFixed(2) || '0.00'}
          </div>
          <div className="text-xs text-gray-500">$/day decay</div>
        </div>

        <div className="p-4 border rounded-lg">
          <div className="text-sm text-gray-600">Net Vega</div>
          <div className="text-2xl font-bold">
            {portfolioGreeks?.net_vega?.toFixed(2) || '0.00'}
          </div>
          <div className="text-xs text-gray-500">
            ${portfolioGreeks?.vega_dollars?.toFixed(0) || '0'} per 1% IV
          </div>
        </div>

        <div className="p-4 border rounded-lg">
          <div className="text-sm text-gray-600">Net Rho</div>
          <div className="text-2xl font-bold">
            {portfolioGreeks?.net_rho?.toFixed(4) || '0.0000'}
          </div>
          <div className="text-xs text-gray-500">per 1% rate</div>
        </div>
      </div>

      {/* Risk Contributions */}
      <div className="border rounded-lg p-4 mb-6">
        <h2 className="text-lg font-semibold mb-4">Risk Contributions</h2>
        <p className="text-gray-600">
          Position-level risk breakdown will be displayed here.
        </p>
      </div>

      {/* Concentration Risks */}
      <div className="border rounded-lg p-4">
        <h2 className="text-lg font-semibold mb-4">Concentration Risks</h2>
        <p className="text-gray-600">
          Alerts for positions with >50% risk contribution.
        </p>
      </div>
    </div>
  );
}