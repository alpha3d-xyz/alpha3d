import { test, expect } from '@playwright/test';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Helper function to perform login
async function loginUser(page) {
  const email = `test-${Date.now()}@example.com`;
  const password = 'testpassword123';
  
  // Step 1: Sign up
  await page.goto('/signup');
  await page.fill('input[type="email"]', email);
  await page.fill('input[type="password"]', password);
  await page.click('button[type="submit"]');
  
  // Wait for signup to complete and redirect to login
  await page.waitForURL('**/login', { timeout: 5000 });
  
  // Step 2: Login
  await page.fill('input[type="email"]', email);
  await page.fill('input[type="password"]', password);
  await page.click('button[type="submit"]');
  
  // Wait for successful login and redirect to home
  await page.waitForURL('**/', { timeout: 5000 });
  await expect(page.locator('h1:has-text("Analyze Geometry")')).toBeVisible({ timeout: 5000 });
}

test.describe('STL Viewer', () => {
  test.beforeEach(async ({ page }) => {
    // Perform login before each test
    await loginUser(page);
  });

  test('should display 3D viewer immediately after STL file upload', async ({ page }) => {
    // Prepare test STL file path
    const testFilePath = path.join(__dirname, '../../data/stl/Cube_3d_printing_sample.stl');
    
    // Set up file chooser handler before triggering the click
    const fileChooserPromise = page.waitForEvent('filechooser');
    
    // Click the drop zone to trigger file input
    await page.locator('.drop-zone').click();
    
    const fileChooser = await fileChooserPromise;
    await fileChooser.setFiles(testFilePath);
    
    // Wait for file to be processed
    // The viewer should appear immediately after file selection
    await page.waitForTimeout(500); // Brief wait for file processing
    
    // Verify the viewer container is visible
    const viewerContainer = page.locator('.viewer-container');
    await expect(viewerContainer).toBeVisible({ timeout: 5000 });
    
    // Verify the viewer has a canvas element (Three.js renders to canvas)
    const canvas = viewerContainer.locator('canvas');
    await expect(canvas).toBeVisible({ timeout: 3000 });
    
    // Verify the analysis results are displayed
    await expect(page.locator('text=Analysis Results')).toBeVisible();
    
    // Verify geometry data is shown
    await expect(page.locator('text=Volume (cm³)')).toBeVisible();
    await expect(page.locator('text=Surface Area (cm²)')).toBeVisible();
  });

  test('should show 3D model with proper rendering', async ({ page }) => {
    const testFilePath = path.join(__dirname, '../../data/stl/Cube_3d_printing_sample.stl');
    
    const fileChooserPromise = page.waitForEvent('filechooser');
    await page.locator('.drop-zone').click();
    const fileChooser = await fileChooserPromise;
    await fileChooser.setFiles(testFilePath);
    
    // Wait for viewer
    const viewerContainer = page.locator('.viewer-container');
    await expect(viewerContainer).toBeVisible({ timeout: 5000 });
    
    // Check that canvas has non-zero dimensions
    const canvas = viewerContainer.locator('canvas');
    const box = await canvas.boundingBox();
    
    expect(box).not.toBeNull();
    expect(box.width).toBeGreaterThan(0);
    expect(box.height).toBeGreaterThan(0);
  });

  test('should allow user to interact with 3D model', async ({ page }) => {
    const testFilePath = path.join(__dirname, '../../data/stl/Cube_3d_printing_sample.stl');
    
    const fileChooserPromise = page.waitForEvent('filechooser');
    await page.locator('.drop-zone').click();
    const fileChooser = await fileChooserPromise;
    await fileChooser.setFiles(testFilePath);
    
    // Wait for viewer
    const canvas = page.locator('.viewer-container canvas');
    await expect(canvas).toBeVisible({ timeout: 5000 });
    
    // Simulate mouse interaction (drag to rotate)
    const box = await canvas.boundingBox();
    await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
    await page.mouse.down();
    await page.mouse.move(box.x + box.width / 2 + 50, box.y + box.height / 2 + 50);
    await page.mouse.up();
    
    // If the viewer is working, canvas should still be visible and responsive
    await expect(canvas).toBeVisible();
  });

  test('should clear viewer when analyzing another file', async ({ page }) => {
    const testFilePath = path.join(__dirname, '../../data/stl/Cube_3d_printing_sample.stl');
    
    // Upload first file
    let fileChooserPromise = page.waitForEvent('filechooser');
    await page.locator('.drop-zone').click();
    let fileChooser = await fileChooserPromise;
    await fileChooser.setFiles(testFilePath);
    
    await expect(page.locator('.viewer-container')).toBeVisible({ timeout: 5000 });
    
    // Click "Analyze Another File"
    await page.click('button:has-text("Analyze Another File")');
    
    // Viewer should be hidden
    await expect(page.locator('.viewer-container')).not.toBeVisible();
    
    // Drop zone should be visible again
    await expect(page.locator('.drop-zone')).toBeVisible();
  });
});
