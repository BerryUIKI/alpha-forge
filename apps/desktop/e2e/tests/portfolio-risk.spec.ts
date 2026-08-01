/**
 * Portfolio Risk E2E Tests
 * Tests portfolio-level risk analysis
 */

import { test, expect } from '@playwright/test';

test.describe('Portfolio Risk Analysis', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/options/portfolio');
  });

  test('should load portfolio risk page', async ({ page }) => {
    await expect(page.locator('h1')).toContainText('Portfolio Risk');
  });

  test('should display Greeks cards', async ({ page }) => {
    // Check for Net Delta card
    await expect(page.locator('text=Net Delta')).toBeVisible();
    await expect(page.locator('text=Net Gamma')).toBeVisible();
    await expect(page.locator('text=Net Theta')).toBeVisible();
    await expect(page.locator('text=Net Vega')).toBeVisible();
  });

  test('should show dollar exposure', async ({ page }) => {
    // Check for dollar-denominated values
    await expect(page.locator('text=exposure')).toBeVisible();
  });

  test('should display risk contributions section', async ({ page }) => {
    await expect(page.locator('text=Risk Contributions')).toBeVisible();
  });

  test('should display concentration risks section', async ({ page }) => {
    await expect(page.locator('text=Concentration Risks')).toBeVisible();
  });
});