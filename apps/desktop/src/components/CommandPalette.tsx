/**
 * Command Palette Component
 * Global command palette for quick actions (Cmd+K / Ctrl+K)
 */

export interface Command {
  id: string;
  title: string;
  category?: string;
  action: () => void;
}

export interface CommandPaletteProps {
  onClose: () => void;
}

export function CommandPalette({ onClose }: CommandPaletteProps) {
  return null; // Phase 2 component - skeleton only
}

export default CommandPalette;