use std::fs;
use serde::{Deserialize,Serialize};
use chrono::Utc;
use uuid::Uuid;

#[derive(Deserialize)]
struct Post{
text:String,
images:Vec<String>,
videos:Vec<String>
}

#[derive(Serialize,Deserialize,Clone)]
struct Entry{
id:String,
title:String,
url:String,
date:String,
file:String,
images:usize,
videos:usize
}

fn download(url:&str,path:&str){

if let Ok(resp)=reqwest::blocking::get(url){

if let Ok(bytes)=resp.bytes(){

let _=fs::write(path,bytes);

}

}

}

fn main(){

fs::create_dir_all("archive/posts").unwrap();
fs::create_dir_all("archive/assets/images").unwrap();
fs::create_dir_all("archive/assets/videos").unwrap();

let data=fs::read_to_string("posts.json").unwrap();

let posts:Vec<Post>=serde_json::from_str(&data).unwrap();

let mut md=String::new();

let mut img_total=0;
let mut vid_total=0;

for (i,p) in posts.iter().enumerate(){

md.push_str(&p.text);
md.push_str("\n\n");

for (j,img) in p.images.iter().enumerate(){

let path=format!("archive/assets/images/{}_{}.jpg",i,j);

download(img,&path);

md.push_str(&format!("![](/{} )\n\n",path));

img_total+=1;

}

for (j,vid) in p.videos.iter().enumerate(){

let path=format!("archive/assets/videos/{}_{}.mp4",i,j);

download(vid,&path);

md.push_str(&format!(
"<video controls src=\"/{}\" width=\"600\"></video>\n\n",
path
));

vid_total+=1;

}

}

let id=Uuid::new_v4().to_string();

let file=format!("archive/posts/{}.md",&id);

fs::write(&file,md).unwrap();

let meta_path="archive/meta.json";

let mut meta:Vec<Entry>=if let Ok(s)=fs::read_to_string(meta_path){

serde_json::from_str(&s).unwrap_or_default()

}else{

Vec::new()

};

meta.push(Entry{

id:id.clone(),
title:format!("Archive {}",id),
url:"".into(),
date:Utc::now().to_rfc3339(),
file:file.clone(),
images:img_total,
videos:vid_total

});

fs::write(meta_path,serde_json::to_string_pretty(&meta).unwrap()).unwrap();

}
