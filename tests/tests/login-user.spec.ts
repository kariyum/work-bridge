import { test, expect } from '@playwright/test';
import { createTestAccount, deleteTestAccount } from '../account-management/accountManager';
import { UserAccount } from '../models/user-account';
let testingAccounts: UserAccount[] = [];

let recruiterUserAccount: UserAccount;
let freelancerUserAccount: UserAccount;
test.describe('Registration Page', () => {
  test.beforeAll(async () => {
    recruiterUserAccount = await createTestAccount('recruiter');
    freelancerUserAccount = await createTestAccount('freelancer');
    testingAccounts.push(recruiterUserAccount, freelancerUserAccount);
  });

  test.afterAll(async () => {
    testingAccounts.forEach(async (account) => {
      await deleteTestAccount(account.email);
    });
  });

  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:5173/login', { waitUntil: 'networkidle' });
  });

  test('should login recruiter', async ({ page }) => {
		await page.getByPlaceholder('Email').fill(recruiterUserAccount.email);
    await page.getByPlaceholder('Password').fill(recruiterUserAccount.password);
    await page.getByRole('button', { name: /login/i }).click();
    await expect(page).toHaveTitle('Landing Page Recruiter');
    await page.getByTestId('logout-button').click();
		await page.close();
	});

  test('should login freelancer', async ({ page }) => {
		await page.getByPlaceholder('Email').fill(freelancerUserAccount.email);
    await page.getByPlaceholder('Password').fill(freelancerUserAccount.password);
    await page.getByRole('button', { name: /login/i }).click();
    await expect(page).toHaveTitle('Landing Page Freelancer');
    await page.getByTestId('logout-button').click();
		await page.close();
	});
});