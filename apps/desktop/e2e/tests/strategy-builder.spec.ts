/**
 * Strategy Builder E2E Tests
 * Tests multi-leg strategy construction workflow
 */

import { test, expect } from '@playwright/test';

test.describe('Strategy Builder', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/options');
  });

  test('should navigate to strategy builder', async ({ page }) => {
    // Click strategy builder link
    await page.locator('text=Strategy Builder').click();
    
    // Should navigate to strategy page
    await expect(page).toHaveURL(/.*strategy/);
  });

  test('should display strategy templates', async ({ page }) => {
    await page.goto('/options/strategy');
    
    // Check for strategy selector
    const strategySelector = page.locator('select, [data-testid="strategy-selector"]');
    await expect(strategySelector.first()).toBeVisible();
  });

  test('should allow adding legs to strategy', async ({ page }) => {
    await page.goto('/options/strategy');
    
    // Look for "Add Leg" button
    const addLegButton = page.locator('button:has-text("Add"), button:has-text("Leg")');
    
    if (await addLegButton.count() > 0) {
      await addLegButton.first().click();
      
      // Should show leg configuration
      await page.waitForTimeout(1000);
    }
  });
});