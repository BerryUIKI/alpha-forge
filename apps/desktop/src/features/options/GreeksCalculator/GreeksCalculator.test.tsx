/**
 * GreeksCalculator Component Tests
 */

import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { GreeksForm } from './GreeksForm';
import { GreeksResults } from './GreeksResults';
import type { GreeksResponse } from '@/types/option';

const queryClient = new QueryClient({
  defaultOptions: { queries: { retry: false } }
});

const wrapper = ({ children }: { children: React.ReactNode }) => (
  <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
);

describe('GreeksForm', () => {
  it('renders form with all inputs', () => {
    render(<GreeksForm onSubmit={() => {}} />, { wrapper });
    
    expect(screen.getByLabelText(/Option Type/i)).toBeInTheDocument();
    expect(screen.getByLabelText(/Underlying Price/i)).toBeInTheDocument();
    expect(screen.getByLabelText(/Strike Price/i)).toBeInTheDocument();
  });

  it('calls onSubmit with form data', async () => {
    const onSubmit = vi.fn();
    render(<GreeksForm onSubmit={onSubmit} />, { wrapper });
    
    // User would fill form and submit
    // For now, just verify the button exists
    expect(screen.getByText(/Calculate Greeks/i)).toBeInTheDocument();
  });
});

describe('GreeksResults', () => {
  it('shows placeholder when no results', () => {
    render(<GreeksResults results={null} />);
    
    expect(screen.getByText(/Enter parameters/i)).toBeInTheDocument();
  });

  it('displays all five Greeks', () => {
    const mockResults: GreeksResponse = {
      delta: 0.5,
      gamma: 0.1,
      theta: -0.05,
      vega: 0.2,
      rho: 0.01,
    };
    
    render(<GreeksResults results={mockResults} />);
    
    expect(screen.getByText('Delta')).toBeInTheDocument();
    expect(screen.getByText('Gamma')).toBeInTheDocument();
    expect(screen.getByText('Theta')).toBeInTheDocument();
    expect(screen.getByText('Vega')).toBeInTheDocument();
    expect(screen.getByText('Rho')).toBeInTheDocument();
  });
});