const {chromium}=require("playwright");
const fs=require("fs");

(async()=>{

const url=process.argv[2];

const browser=await chromium.launch();

const page=await browser.newPage();

await page.goto(url,{waitUntil:"networkidle"});

await page.waitForTimeout(5000);

const posts=await page.evaluate(()=>{

const arts=[...document.querySelectorAll("article")];

if(arts.length===0){

return [{
text:document.body.innerText,
images:[...document.images].map(i=>i.src),
videos:[...document.querySelectorAll("video source")].map(v=>v.src)
}];

}

return arts.map(a=>{

return{

text:a.innerText,
images:[...a.querySelectorAll("img")].map(i=>i.src),
videos:[...a.querySelectorAll("video source")].map(v=>v.src)

}

})

});

fs.writeFileSync("posts.json",JSON.stringify(posts,null,2));

await browser.close();

})();
