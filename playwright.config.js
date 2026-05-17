const { defineConfig } = require("@playwright/test");

module.exports = defineConfig({
  testDir: "./e2e",
  fullyParallel: false,
  retries: 0,
  reporter: "list",
  use: {
    baseURL: "http://127.0.0.1:8080",
    headless: true,
    trace: "retain-on-failure"
  },
  webServer: {
    command: "python3 -m http.server 8080 --bind 0.0.0.0 --directory dist",
    port: 8080,
    reuseExistingServer: true,
    timeout: 30000
  }
});
