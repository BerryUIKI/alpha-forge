/**
 * Greeks Calculator E2E Tests
 * Tests Greeks calculation workflow
 */

import { test, expect } from '@playwright/test';

test.describe('Greeks Calculator', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/options/greeks');
  });

  test('should load Greeks calculator page', async ({ page }) => {
    await expect(page.locator('h1')).toContainText('Greeks Calculator');
  });

  test('should display all input fields', async ({ page }) => {
    // Check for essential input fields
    await expect(page.locator('select')).toBeVisible(); // Option Type
    await expect(page.locator('input[value="100"]')).toBeVisible(); // Underlying Price
  });

  test('should calculate Greeks on form submission', async ({ page }) => {
    // Fill form with default values (already populated)
    const submitButton = page.locator('button:has-text("Calculate Greeks")');
    await submitButton.click();
    
    // Should show results
    await page.waitForTimeout(1000);
    
    // Check for Greeks results
    const deltaResult = page.locator('text=Delta');
    await expect(deltaResult).toBeVisible({ timeout: 5000 });
  });

  test('should display all five Greeks', async ({ page }) => {
    const submitButton = page.locator('button:has-text("Calculate Greeks")');
    await submitButton.click();
    
    await page.waitForTimeout(2000);
    
    // Check for all Greeks
    await expect(page.locator('text=Delta')).toBeVisible();
    await expect(page.locator('text=Gamma')).toBeVisible();
    await expect(page.locator('text=Theta')).toBeVisible();
    await expect(page.locator('text=Vega')).toBeVisible();
    await expect(page.locator('text=Rho')).toBeVisible();
  });

  test('should update chart visualization', async ({ page }) => {
    const submitButton = page.locator('button:has-text("Calculate Greeks")');
    await submitButton.click();
    
    await page.waitForTimeout(2000);
    
    // Check for chart container
    const chart = page.locator('.payoff-diagram, canvas, svg');
    await expect(chart.first()).toBeVisible({ timeout: 5000 });
  });
});