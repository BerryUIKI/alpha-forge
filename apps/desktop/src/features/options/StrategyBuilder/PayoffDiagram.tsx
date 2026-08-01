/**
 * PayoffDiagram Component
 * Interactive P&L visualization using Chart.js
 */

import { useEffect, useRef } from 'react';
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  Filler,
} from 'chart.js';
import { Line } from 'react-chartjs-2';

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  Filler
);

export interface PayoffDiagramProps {
  legs: Array<{
    strike: number;
    optionType: 'call' | 'put';
    positionType: 'long' | 'short';
    quantity: number;
    premium: number;
  }>;
  underlyingPrice: number;
  breakEvenPoints?: number[];
  maxProfit?: number;
  maxLoss?: number;
}

export function PayoffDiagram({
  legs,
  underlyingPrice,
  breakEvenPoints = [],
  maxProfit,
  maxLoss,
}: PayoffDiagramProps) {
  // Calculate payoff for range of prices
  const priceRange = [];
  const payoffs = [];
  
  const minPrice = Math.max(0, underlyingPrice * 0.5);
  const maxPrice = underlyingPrice * 1.5;
  const step = (maxPrice - minPrice) / 100;

  for (let price = minPrice; price <= maxPrice; price += step) {
    priceRange.push(price.toFixed(2));
    
    // Calculate total payoff at this price
    let totalPayoff = 0;
    
    for (const leg of legs) {
      const { strike, optionType, positionType, quantity, premium } = leg;
      let payoff = 0;

      // Intrinsic value at expiration
      if (optionType === 'call') {
        payoff = Math.max(0, price - strike);
      } else {
        payoff = Math.max(0, strike - price);
      }

      // Adjust for position type
      if (positionType === 'short') {
        payoff = -payoff;
        totalPayoff += (payoff + premium) * quantity;
      } else {
        totalPayoff += (payoff - premium) * quantity;
      }
    }

    payoffs.push(totalPayoff);
  }

  const chartData = {
    labels: priceRange,
    datasets: [
      {
        label: 'P&L',
        data: payoffs,
        borderColor: 'rgb(59, 130, 246)',
        backgroundColor: (context: any) => {
          const value = context.raw;
          return value >= 0 ? 'rgba(34, 197, 94, 0.3)' : 'rgba(239, 68, 68, 0.3)';
        },
        fill: true,
        tension: 0.1,
        pointRadius: 0,
      },
    ],
  };

  const options = {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: {
        display: false,
      },
      title: {
        display: true,
        text: 'Strategy Payoff at Expiration',
        font: { size: 16 },
      },
      tooltip: {
        callbacks: {
          label: (context: any) => {
            const value = context.parsed.y;
            return `P&L: $${value.toFixed(2)}`;
          },
        },
      },
    },
    scales: {
      x: {
        title: {
          display: true,
          text: 'Underlying Price at Expiration',
        },
      },
      y: {
        title: {
          display: true,
          text: 'Profit / Loss ($)',
        },
        grid: {
          color: (context: any) => {
            return context.tick.value === 0 ? 'rgba(0, 0, 0, 0.3)' : 'rgba(0, 0, 0, 0.1)';
          },
        },
      },
    },
    annotation: {
      annotations: {
        currentPrice: {
          type: 'line',
          xMin: underlyingPrice,
          xMax: underlyingPrice,
          borderColor: 'rgb(239, 68, 68)',
          borderWidth: 2,
          label: {
            display: true,
            content: `Current: $${underlyingPrice}`,
          },
        },
      },
    },
  };

  return (
    <div className="payoff-diagram">
      <div className="h-96">
        <Line data={chartData} options={options} />
      </div>
      
      <div className="grid grid-cols-3 gap-4 mt-4">
        {breakEvenPoints.length > 0 && (
          <div className="p-3 border rounded">
            <div className="text-sm text-gray-600">Break-Even Points</div>
            <div className="font-semibold">
              {breakEvenPoints.map((be) => `$${be.toFixed(2)}`).join(', ')}
            </div>
          </div>
        )}
        
        {maxProfit !== undefined && (
          <div className="p-3 border rounded">
            <div className="text-sm text-gray-600">Max Profit</div>
            <div className="font-semibold text-green-600">
              {maxProfit === Infinity ? 'Unlimited' : `$${maxProfit.toFixed(2)}`}
            </div>
          </div>
        )}
        
        {maxLoss !== undefined && (
          <div className="p-3 border rounded">
            <div className="text-sm text-gray-600">Max Loss</div>
            <div className="font-semibold text-red-600">
              {maxLoss === -Infinity ? 'Unlimited' : `$${Math.abs(maxLoss).toFixed(2)}`}
            </div>
          </div>
        )}
      </div>
    </div>
  );
}