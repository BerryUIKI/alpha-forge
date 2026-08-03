import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { desktopApi } from "@/lib/desktop-api";
import type { CreatePortfolioAccountParams, CreatePortfolioPositionParams } from "@/lib/desktop-api/portfolio";

const portfolioKeys = {
  all: ["portfolio"] as const,
  accounts: (workspaceId: string) => [...portfolioKeys.all, "accounts", workspaceId] as const,
  positions: (accountId: string) => [...portfolioKeys.all, "positions", accountId] as const,
  transactions: (accountId: string) => [...portfolioKeys.all, "transactions", accountId] as const,
  allocation: (workspaceId: string) => [...portfolioKeys.all, "allocation", workspaceId] as const,
  risks: (workspaceId: string) => [...portfolioKeys.all, "risks", workspaceId] as const,
  themes: (workspaceId: string) => [...portfolioKeys.all, "themes", workspaceId] as const,
  alignment: (workspaceId: string) => [...portfolioKeys.all, "alignment", workspaceId] as const,
};

export function usePortfolioAccounts(workspaceId: string) {
  return useQuery({ queryKey: portfolioKeys.accounts(workspaceId), queryFn: () => desktopApi.portfolio.listPortfolioAccounts(workspaceId), enabled: Boolean(workspaceId) });
}

export function usePortfolioPositions(accountId: string) {
  return useQuery({ queryKey: portfolioKeys.positions(accountId), queryFn: () => desktopApi.portfolio.listPortfolioPositions(accountId), enabled: Boolean(accountId) });
}
export function usePortfolioTransactions(accountId: string) { return useQuery({ queryKey: portfolioKeys.transactions(accountId), queryFn: () => desktopApi.portfolio.listPortfolioTransactions(accountId), enabled: Boolean(accountId) }); }
export function usePortfolioAllocation(workspaceId: string) { return useQuery({ queryKey: portfolioKeys.allocation(workspaceId), queryFn: () => desktopApi.portfolio.getPortfolioAllocation(workspaceId), enabled: Boolean(workspaceId) }); }
export function usePortfolioConcentrationRisks(workspaceId: string) { return useQuery({ queryKey: portfolioKeys.risks(workspaceId), queryFn: () => desktopApi.portfolio.getPortfolioConcentrationRisks(workspaceId), enabled: Boolean(workspaceId) }); }
export function usePortfolioThemeExposure(workspaceId: string) { return useQuery({ queryKey: portfolioKeys.themes(workspaceId), queryFn: () => desktopApi.portfolio.getPortfolioThemeExposure(workspaceId), enabled: Boolean(workspaceId) }); }
export function usePortfolioThesisAlignment(workspaceId: string) { return useQuery({ queryKey: portfolioKeys.alignment(workspaceId), queryFn: () => desktopApi.portfolio.getPortfolioThesisAlignment(workspaceId), enabled: Boolean(workspaceId) }); }

export function useCreatePortfolioAccount() {
  const queryClient = useQueryClient();
  return useMutation({ mutationFn: (input: CreatePortfolioAccountParams) => desktopApi.portfolio.createPortfolioAccount(input), onSuccess: (_, input) => queryClient.invalidateQueries({ queryKey: portfolioKeys.accounts(input.workspaceId) }) });
}

export function useCreatePortfolioPosition() {
  const queryClient = useQueryClient();
  return useMutation({ mutationFn: (input: CreatePortfolioPositionParams) => desktopApi.portfolio.createPortfolioPosition(input), onSuccess: (_, input) => { queryClient.invalidateQueries({ queryKey: portfolioKeys.positions(input.accountId) }); queryClient.invalidateQueries({ queryKey: portfolioKeys.all }); } });
}
export function useImportPortfolioTransactions() {
  const queryClient = useQueryClient();
  return useMutation({ mutationFn: ({ accountId, csvText }: { accountId: string; csvText: string }) => desktopApi.portfolio.importPortfolioTransactionsCsv(accountId, csvText), onSuccess: (_, input) => { queryClient.invalidateQueries({ queryKey: portfolioKeys.transactions(input.accountId) }); queryClient.invalidateQueries({ queryKey: portfolioKeys.positions(input.accountId) }); queryClient.invalidateQueries({ queryKey: portfolioKeys.all }); } });
}
export function useLinkPortfolioTheme() { const queryClient = useQueryClient(); return useMutation({ mutationFn: ({ workspaceId, symbol, entityId }: { workspaceId: string; symbol: string; entityId: string }) => desktopApi.portfolio.linkPortfolioTheme(workspaceId, symbol, entityId), onSuccess: (_, input) => queryClient.invalidateQueries({ queryKey: portfolioKeys.themes(input.workspaceId) }) }); }
export function usePortfolioReview() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (workspaceId: string) => desktopApi.portfolio.generatePortfolioReview(workspaceId),
    onSuccess: (_, workspaceId) => {
      queryClient.invalidateQueries({ queryKey: portfolioKeys.alignment(workspaceId) });
      queryClient.invalidateQueries({ queryKey: portfolioKeys.risks(workspaceId) });
    },
  });
}
