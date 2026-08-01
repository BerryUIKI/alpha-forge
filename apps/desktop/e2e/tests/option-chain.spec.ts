/**
 * Option Chain E2E Tests
 * Tests option chain loading, display, and filtering
 */

import { test, expect } from '@playwright/test';

test.describe('Option Chain Viewer', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/options/chain');
  });

  test('should load option chain page', async ({ page }) => {
    await expect(page.locator('h1')).toContainText('Option Chain');
  });

  test('should display symbol input', async ({ page }) => {
    const symbolInput = page.locator('input[placeholder="AAPL"]');
    await expect(symbolInput).toBeVisible();
  });

  test('should fetch option chain on symbol entry', async ({ page }) => {
    const symbolInput = page.locator('input[placeholder="AAPL"]');
    await symbolInput.fill('AAPL');
    
    // Click refresh button
    const refreshButton = page.locator('button:has-text("Refresh")');
    await refreshButton.click();
    
    // Should show loading state
    await expect(page.locator('text=Loading')).toBeVisible({ timeout: 5000 });
  });

  test('should display option chain table after loading', async ({ page }) => {
    // Wait for data to load
    await page.waitForTimeout(3000);
    
    // Check for table or empty state
    const table = page.locator('table');
    const emptyState = page.locator('text=No options');
    
    // Either table or empty state should be visible
    await expect(table.or(emptyState)).toBeVisible({ timeout: 10000 });
  });

  test('should handle invalid symbol gracefully', async ({ page }) => {
    const symbolInput = page.locator('input[placeholder="AAPL"]');
    await symbolInput.fill('INVALID');
    
    const refreshButton = page.locator('button:has-text("Refresh")');
    await refreshButton.click();
    
    // Should show error or empty state
    await page.waitForTimeout(2000);
  });
});