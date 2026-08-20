/**
 * Workspace hooks barrel export.
 * Separates re-exports from component files to satisfy react-refresh.
 */

export { ActiveWorkspaceProvider } from "./useActiveWorkspace";
export {
  useActiveWorkspace,
  useActiveWorkspaceId,
} from "./useActiveWorkspace.context";
export type { ActiveWorkspaceContextValue } from "./useActiveWorkspace.context";
