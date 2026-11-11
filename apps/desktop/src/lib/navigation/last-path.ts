/**
 * Navigation memory for leaving the Settings screen.
 *
 * The application routes live under MainLayout, while `/settings` is rendered
 * by a separate layout. MainLayout records the last non-settings location so
 * Settings can return there in one action instead of walking back through
 * section-navigation history.
 */

const LAST_APP_PATH_KEY = "last-non-settings-path";

export function rememberAppPath(pathnameWithSearch: string): void {
  if (!pathnameWithSearch.startsWith("/settings")) {
    sessionStorage.setItem(LAST_APP_PATH_KEY, pathnameWithSearch);
  }
}

export function readLastAppPath(): string {
  return sessionStorage.getItem(LAST_APP_PATH_KEY) || "/";
}
