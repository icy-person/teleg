const { chromium } = require("playwright");
const fs = require("fs");

(async () => {

const url = process.argv[2];

const browser = await chromium.launch();

const page = await browser.newPage();

await page.goto(url,{waitUntil:"networkidle"});

await page.waitForTimeout(4000);

const posts = await page.evaluate(()=>{

const articles = Array.from(document.querySelectorAll("article"));

if(articles.length===0){

return [{
text:document.body.innerText,
images:Array.from(document.images).map(i=>i.src),
videos:Array.from(document.querySelectorAll("video,source")).map(v=>v.src)
}];

}

return articles.map(a=>{

return{

text:a.innerText,

images:Array.from(a.querySelectorAll("img")).map(i=>i.src),

videos:Array.from(a.querySelectorAll("video,source"))
.map(v=>v.src)
.filter(Boolean)

}

})

});

fs.writeFileSync("posts.json",JSON.stringify(posts,null,2));

await browser.close();

})();
