import { test, expect } from '@playwright/test';

test.describe('Landing Page', () => {
    test.beforeEach(async ({ page }) => {
      await page.goto('http://localhost:5173/', { waitUntil: 'networkidle' });
    });
  
    test('should navigate to the login page when clicking the login button', async ({ page }) => {
        await expect(page).toHaveTitle(/Landing Page/);
        const loginButton = page.getByTestId('landing-page-login-button');
        await expect(loginButton).toBeVisible();
  
        await loginButton.click();
        await expect(page).toHaveURL('http://localhost:5173/login');
    });
  
    test('should navigate to the registration page when clicking the register button', async ({
      page,
    }) => {
        const registerButton = page.getByTestId('landing-page-registration-button');
        await expect(registerButton).toBeVisible();

        registerButton.click();
        await expect(page).toHaveURL('http://localhost:5173/register');
        await page.close();
    });
});