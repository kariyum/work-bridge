import { deleteTestAccount } from '../account-management/accountManager';
import { expect, test } from '@playwright/test';

test.describe('Registration Page', () => {
	test.beforeEach(async ({ page }) => {
        await page.goto('http://localhost:5173/register', { waitUntil: 'networkidle' });
	});

	test('should allow selecting a role', async ({ page }) => {
		await page.getByTestId('role-recruiter').click();
		await expect(page.locator('#recruiter')).toBeChecked();

		await page.getByTestId('role-freelancer').click();
		await expect(page.locator('#freelancer')).toBeChecked();
	});

	test('should navigate to the user form after selecting a role', async ({ page }) => {
		await page.getByTestId('role-recruiter').click();
		await page.getByRole('button', { name: /continue/i }).click();
		await expect(page.getByTestId('first-name')).toBeVisible();
	});

	test('should navigate back to the role picker from the user form', async ({ page }) => {
		await page.getByTestId('role-recruiter').click();
		await page.getByRole('button', { name: /continue/i }).click();
		await page.getByTestId('previous-button').click();
		await expect(page.getByTestId('role-recruiter')).toBeVisible();
	});

	test('should redirect to the login page', async ({ page }) => {
		await page.getByTestId('login-link').click();
		await expect(page).toHaveURL('http://localhost:5173/login');
	});

	test('should redirect to the home page', async ({ page }) => {
		await page.getByTestId('previous-button').click();
		await expect(page).toHaveURL('http://localhost:5173/');
	});

	test('should successfully register a new recruiter', async ({ page }) => {
        const accountId = `test_${Date.now()}@gmail.com`;
		await page.getByTestId('role-recruiter').click();
		await page.getByRole('button', { name: /continue/i }).click();

		await page.getByPlaceholder('Name', { exact: true }).fill('Test');
		await page.getByPlaceholder('Last Name', { exact: true }).fill('User');
		await page.getByPlaceholder('Email', { exact: true }).fill(accountId);
		await page.getByPlaceholder('Password', { exact: true }).fill('password123');
		await page.getByPlaceholder('Confirm password', { exact: true }).fill('password123');

		await page.getByRole('button', { name: /submit/i }).click();

		await expect(page).toHaveTitle('Landing Page Recruiter');
		await page.close();
		await deleteTestAccount('testuser@example.com');
	});

	test('should successfully register a new freelancer', async ({ page }) => {
        let accountId = `test_${Date.now()}@gmail.com`;
		await page.getByTestId('role-freelancer').click();
		await page.getByRole('button', { name: /continue/i }).click();

		await expect(page.getByTestId('first-name')).toBeVisible();

		await page.getByPlaceholder('Name', { exact: true }).fill('Test');
		await page.getByPlaceholder('Last Name', { exact: true }).fill('User');
		await page.getByPlaceholder('Email', { exact: true }).fill(accountId);
		await page.getByPlaceholder('Password', { exact: true }).fill('password123');
		await page.getByPlaceholder('Confirm password', { exact: true }).fill('password123');

		await page.getByRole('button', { name: /submit/i }).click();

		await expect(page).toHaveTitle('Landing Page Freelancer');
		await page.close();
		await deleteTestAccount('testuser@example.com');
	});

	test('should navigate to home using the home link', async ({ page }) => {
		await page.getByTestId('home-link').click();
		await expect(page).toHaveURL('http://localhost:5173/');
        await page.close();
	});
});