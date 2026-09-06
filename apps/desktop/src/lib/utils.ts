/**
 * Utility functions
 */

import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

/**
 * Detects whether the runtime host is macOS.
 */
export function isMacPlatform(): boolean {
  if (typeof navigator !== "undefined") {
    return /macintosh|mac os x/i.test(navigator.userAgent);
  }
  return false;
}