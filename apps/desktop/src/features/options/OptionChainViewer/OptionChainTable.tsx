/**
 * Option Chain Table - Displays contracts in a table format
 */

import { useMemo } from 'react';
import { cn } from '@/lib/utils';
import type { OptionContract } from '@/types/option';

interface OptionChainTableProps {
  contracts: OptionContract[];
  underlyingPrice: number;
}

export function OptionChainTable({ 
  contracts, 
  underlyingPrice 
}: OptionChainTableProps) {
  // Separate calls and puts
  const { calls, puts } = useMemo(() => {
    const calls = contracts.filter(c => c.optionType === 'call');
    const puts = contracts.filter(c => c.optionType === 'put');
    return { calls, puts };
  }, [contracts]);

  // Group by strike
  const groupedContracts = useMemo(() => {
    const map = new Map<number, { call?: OptionContract; put?: OptionContract }>();
    
    calls.forEach(call => {
      const existing = map.get(call.strike) || {};
      map.set(call.strike, { ...existing, call });
    });
    
    puts.forEach(put => {
      const existing = map.get(put.strike) || {};
      map.set(put.strike, { ...existing, put });
    });
    
    return Array.from(map.entries())
      .sort(([a], [b]) => a - b)
      .map(([strike, contracts]) => ({ strike, ...contracts }));
  }, [calls, puts]);

  return (
    <div 
      className="overflow-x-auto"
      role="table"
      aria-label="Option chain data"
    >
      <table className="w-full text-sm">
        <thead className="bg-gray-50 sticky top-0">
          <tr>
            {/* Calls side */}
            <th colSpan={5} className="text-center py-2 border-b">
              Calls
            </th>
            {/* Strike */}
            <th className="text-center py-2 border-b bg-gray-100">
              Strike
            </th>
            {/* Puts side */}
            <th colSpan={5} className="text-center py-2 border-b">
              Puts
            </th>
          </tr>
          <tr>
            {/* Call columns */}
            <th className="px-3 py-2 text-right">Bid</th>
            <th className="px-3 py-2 text-right">Ask</th>
            <th className="px-3 py-2 text-right">Vol</th>
            <th className="px-3 py-2 text-right">OI</th>
            <th className="px-3 py-2 text-right">IV</th>
            {/* Strike */}
            <th className="px-3 py-2 text-center bg-gray-100">Strike</th>
            {/* Put columns */}
            <th className="px-3 py-2 text-right">IV</th>
            <th className="px-3 py-2 text-right">OI</th>
            <th className="px-3 py-2 text-right">Vol</th>
            <th className="px-3 py-2 text-right">Ask</th>
            <th className="px-3 py-2 text-right">Bid</th>
          </tr>
        </thead>
        <tbody>
          {groupedContracts.map(({ strike, call, put }) => {
            const isATM = Math.abs(strike - underlyingPrice) < 2;
            
            return (
              <tr 
                key={strike}
                className={cn(
                  "hover:bg-gray-50",
                  isATM && "bg-blue-50 font-medium"
                )}
              >
                {/* Call side */}
                <td className="px-3 py-2 text-right">
                  {call?.bid?.toFixed(2) || '-'}
                </td>
                <td className="px-3 py-2 text-right">
                  {call?.ask?.toFixed(2) || '-'}
                </td>
                <td className="px-3 py-2 text-right text-gray-600">
                  {call?.volume?.toLocaleString() || '-'}
                </td>
                <td className="px-3 py-2 text-right text-gray-600">
                  {call?.openInterest?.toLocaleString() || '-'}
                </td>
                <td className="px-3 py-2 text-right text-gray-600">
                  {call?.impliedVolatility 
                    ? `${(call.impliedVolatility * 100).toFixed(1)}%` 
                    : '-'}
                </td>
                
                {/* Strike */}
                <td className="px-3 py-2 text-center bg-gray-50 font-medium">
                  {strike.toFixed(2)}
                </td>
                
                {/* Put side */}
                <td className="px-3 py-2 text-right text-gray-600">
                  {put?.impliedVolatility 
                    ? `${(put.impliedVolatility * 100).toFixed(1)}%` 
                    : '-'}
                </td>
                <td className="px-3 py-2 text-right text-gray-600">
                  {put?.openInterest?.toLocaleString() || '-'}
                </td>
                <td className="px-3 py-2 text-right text-gray-600">
                  {put?.volume?.toLocaleString() || '-'}
                </td>
                <td className="px-3 py-2 text-right">
                  {put?.ask?.toFixed(2) || '-'}
                </td>
                <td className="px-3 py-2 text-right">
                  {put?.bid?.toFixed(2) || '-'}
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}

export default OptionChainTable;