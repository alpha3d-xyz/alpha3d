import { test, expect } from '@playwright/test';

test.describe('Admin Dashboard', () => {
  test('should display admin link and dashboard for admin users', async ({ page }) => {
    // Mock the auth/me endpoint to return an admin user
    await page.route('**/api/auth/me', async route => {
      const json = {
        id: '123',
        email: 'admin@example.com',
        role: 'ADMIN',
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString()
      };
      await route.fulfill({ json });
    });

    // Mock the admin orders endpoint
    await page.route('**/api/admin/orders', async route => {
      const json = [
        {
          id: 'order-1',
          user_id: 'user-1',
          quote_id: 'quote-1',
          status: 'PAID',
          shipping_address: {},
          created_at: new Date().toISOString()
        }
      ];
      await route.fulfill({ json });
    });

    // Login (Mock login response too to avoid real backend call if needed, but we can just bypass login UI if we set state)
    // Or just go through login flow and let the real login happen, but intercept the `me` call.
    
    // Let's use a helper to set the auth state directly if possible, or just mock the login.
    await page.route('**/api/auth/login', async route => {
      await route.fulfill({ json: { token: 'fake-jwt-token' } });
    });

    await page.goto('/login');
    await page.fill('input[type="email"]', 'admin@example.com');
    await page.fill('input[type="password"]', 'password');
    await page.click('button[type="submit"]');
    
    // Wait for navigation to home
    await page.waitForURL('**/');

    // Expect Admin link to be visible
    const adminLink = page.locator('a[href="/admin"]');
    await expect(adminLink).toBeVisible();

    // Click Admin link
    await adminLink.click();
    await page.waitForURL('**/admin');

    // Expect to see the order
    await expect(page.locator('text=order-1')).toBeVisible();
    
    // Check if the select has the correct value
    const statusSelect = page.locator('select.status-select');
    await expect(statusSelect).toBeVisible();
    await expect(statusSelect).toHaveValue('PAID');
  });

  test('should NOT display admin link for normal users', async ({ page }) => {
    // Mock the auth/me endpoint to return a normal user
    await page.route('**/api/auth/me', async route => {
      const json = {
        id: '123',
        email: 'user@example.com',
        role: 'USER',
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString()
      };
      await route.fulfill({ json });
    });

    await page.route('**/api/auth/login', async route => {
      await route.fulfill({ json: { token: 'fake-jwt-token' } });
    });

    await page.goto('/login');
    await page.fill('input[type="email"]', 'user@example.com');
    await page.fill('input[type="password"]', 'password');
    await page.click('button[type="submit"]');
    
    await page.waitForURL('**/');

    // Expect Admin link to NOT be visible
    const adminLink = page.locator('a[href="/admin"]');
    await expect(adminLink).not.toBeVisible();
  });
});
