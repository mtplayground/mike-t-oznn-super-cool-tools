const { test, expect } = require("@playwright/test");

test("calculator card loads and evaluates a simple expression", async ({ page }) => {
  await page.goto("/");

  const calculatorCard = page.getByRole("link", { name: /calculator/i }).first();
  await expect(calculatorCard).toBeVisible();

  await calculatorCard.click();
  await expect(page).toHaveURL(/\/tools\/calculator$/);

  const display = page.locator("[data-role='display']");
  await expect(display).toHaveText("0");

  await page.locator("[data-input='digit-2']").click();
  await page.locator("[data-input='add']").click();
  await page.locator("[data-input='digit-3']").click();
  await page.locator("[data-input='equals']").click();

  await expect(display).toHaveText("5");
});
