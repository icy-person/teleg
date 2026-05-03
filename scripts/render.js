const { chromium } = require("playwright");
const fs = require("fs");

(async () => {
  const url = process.argv[2];

  const browser = await chromium.launch({
    headless: true
  });

  const page = await browser.newPage();

  await page.goto(url, {
    waitUntil: "networkidle",
    timeout: 60000
  });

  // کمی صبر برای لود JS
  await page.waitForTimeout(4000);

  const html = await page.content();

  fs.writeFileSync("page.html", html);

  await browser.close();
})();
