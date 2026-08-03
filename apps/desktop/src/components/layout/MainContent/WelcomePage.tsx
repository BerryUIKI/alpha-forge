/**
 * WelcomePage Component
 *
 * Welcome screen displayed when no project/session is selected.
 * Provides a clean, inviting interface for new users.
 *
 * @version GUI-M1-4
 */

import { FolderOpen, Search, BarChart3, TrendingUp } from "lucide-react";

const QUICK_ACTIONS = [
  { icon: FolderOpen, label: "Open Project", description: "Load existing research project" },
  { icon: Search, label: "New Research", description: "Start a new investigation" },
  { icon: BarChart3, label: "Analyze Market", description: "Run market analysis tools" },
  { icon: TrendingUp, label: "View Portfolio", description: "Review your holdings" },
];

export function WelcomePage() {
  return (
    <div className="flex h-full flex-col items-center justify-center p-8">
      {/* Hero Section */}
      <div className="mb-12 text-center">
        <h1 className="text-4xl font-bold mb-4">
          Welcome to Investment OS
        </h1>
        <p className="text-lg text-muted-foreground max-w-md">
          Your AI-powered investment research workspace
        </p>
      </div>

      {/* Quick Actions Grid */}
      <div className="grid grid-cols-2 gap-4 max-w-lg w-full">
        {QUICK_ACTIONS.map((action) => {
          const Icon = action.icon;
          return (
            <button
              key={action.label}
              className="flex flex-col items-start p-4 rounded-lg border border-border hover:bg-accent transition-colors text-left disabled:opacity-50"
              disabled
              title={`${action.label} (coming soon)`}
            >
              <Icon className="h-6 w-6 mb-2 text-primary" />
              <h3 className="font-medium text-sm">{action.label}</h3>
              <p className="text-xs text-muted-foreground mt-1">
                {action.description}
              </p>
            </button>
          );
        })}
      </div>

      {/* Getting Started Hint */}
      <div className="mt-12 text-center">
        <p className="text-sm text-muted-foreground">
          Select a workspace from the left sidebar to begin
        </p>
        <p className="text-xs text-muted-foreground mt-1">
          Or use keyboard shortcuts: <kbd className="px-1.5 py-0.5 rounded bg-muted">Ctrl+1-6</kbd>
        </p>
      </div>

      {/* TODO markers for future implementation */}
      {/* TODO: [GUI-M1-4] Add actual quick action handlers */}
      {/* TODO: [GUI-M1-4] Add recent projects list */}
      {/* TODO: [GUI-M1-4] Add workspace statistics */}
    </div>
  );
}