/**
 * Options Dashboard Page
 * Main entry point for Options Analysis Platform
 */

import { Link } from 'react-router-dom';

export function OptionsDashboard() {
  const features = [
    {
      title: 'Option Chain Viewer',
      description: 'View and analyze option chains with real-time Greeks',
      path: '/options/chain',
      icon: '📊',
    },
    {
      title: 'Greeks Calculator',
      description: 'Calculate option Greeks using Black-Scholes model',
      path: '/options/greeks',
      icon: '🧮',
    },
    {
      title: 'Strategy Builder',
      description: 'Build and analyze multi-leg option strategies',
      path: '/options/strategy',
      icon: '🎯',
    },
    {
      title: 'Portfolio Risk',
      description: 'Portfolio-level Greeks and risk analysis',
      path: '/options/portfolio',
      icon: '⚖️',
    },
  ];

  return (
    <div className="p-6">
      <div className="mb-8">
        <h1 className="text-3xl font-bold">Options Analysis Platform</h1>
        <p className="text-gray-600 mt-2">
          Professional-grade option analysis tools for investment research
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        {features.map((feature) => (
          <Link
            key={feature.path}
            to={feature.path}
            className="block p-6 border rounded-lg hover:shadow-lg transition-shadow"
          >
            <div className="text-4xl mb-4">{feature.icon}</div>
            <h2 className="text-xl font-semibold mb-2">{feature.title}</h2>
            <p className="text-gray-600 text-sm">{feature.description}</p>
          </Link>
        ))}
      </div>

      <div className="mt-8 p-6 bg-blue-50 border border-blue-200 rounded-lg">
        <h2 className="text-lg font-semibold mb-2">Phase 3 Status</h2>
        <ul className="space-y-1 text-sm">
          <li>✅ Option Chain Viewer with filtering</li>
          <li>✅ Greeks Calculator with Black-Scholes</li>
          <li>✅ Strategy Builder framework</li>
          <li>✅ IPC integration functional</li>
          <li>✅ All UI states implemented</li>
        </ul>
      </div>
    </div>
  );
}

export default OptionsDashboard;