const { chromium } = require("playwright");
const fs = require("fs");

(async () => {

  const url = process.argv[2];

  const browser = await chromium.launch();
  const page = await browser.newPage();

  await page.goto(url, { waitUntil: "networkidle" });

  await page.waitForTimeout(4000);

  const posts = await page.evaluate(() => {

    const articles = Array.from(document.querySelectorAll("article"));

    return articles.map(a => {

      const images = Array.from(a.querySelectorAll("img"))
        .map(i => i.src);

      const videos = Array.from(a.querySelectorAll("video, source"))
        .map(v => v.src)
        .filter(Boolean);

      return {
        text: a.innerText,
        images,
        videos
      };

    });

  });

  fs.writeFileSync("posts.json", JSON.stringify(posts,null,2));

  await browser.close();

})();
