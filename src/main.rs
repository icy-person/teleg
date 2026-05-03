use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use chrono::{Utc, DateTime};

#[derive(Deserialize)]
struct Post {
    text: String,
    images: Vec<String>,
    videos: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Stats {
    posts: usize,
    images: usize,
    videos: usize,
}

#[derive(Serialize, Deserialize, Clone)]
struct Entry {
    id: String,
    title: String,
    r#type: String,
    url: String,
    date: DateTime<Utc>,
    file: String,
    stats: Stats,
}

fn download(url: &str, path: &str) {

    if let Ok(resp) = reqwest::blocking::get(url) {

        if let Ok(bytes) = resp.bytes() {

            let _ = fs::write(path, bytes);

        }

    }

}

fn load_meta(path: &str) -> Vec<Entry> {

    if Path::new(path).exists() {

        let s = fs::read_to_string(path).unwrap_or_default();

        serde_json::from_str(&s).unwrap_or_default()

    } else {

        Vec::new()

    }

}

fn save_meta(path: &str, entries: &Vec<Entry>) {

    let s = serde_json::to_string_pretty(entries).unwrap();

    fs::write(path, s).unwrap();

}

fn build_index(entries: &Vec<Entry>) {

    let mut md = String::new();

    md.push_str("# 🗂 Archive\n\n");
    md.push_str("List of archived pages.\n\n---\n\n");

    let mut list = entries.clone();

    list.sort_by(|a,b| b.date.cmp(&a.date));

    for e in list {

        md.push_str(&format!(
            "- **{}** – [{}]({})  \nType: {} – {} posts – {} images – {} videos\n\n",
            e.date.format("%Y-%m-%d %H:%M"),
            e.title,
            e.file,
            e.r#type,
            e.stats.posts,
            e.stats.images,
            e.stats.videos
        ));

    }

    fs::write("archive/index.md", md).unwrap();

}

fn main() {

    fs::create_dir_all("archive/posts").unwrap();
    fs::create_dir_all("archive/assets/images").unwrap();
    fs::create_dir_all("archive/assets/videos").unwrap();

    let json = fs::read_to_string("posts.json").unwrap();

    let posts: Vec<Post> = serde_json::from_str(&json).unwrap();

    let mut images_total = 0;
    let mut videos_total = 0;

    let mut md = String::new();

    md.push_str("# Archived Page\n\n");

    for (i,p) in posts.iter().enumerate() {

        md.push_str(&p.text);
        md.push_str("\n\n");

        for (j,img) in p.images.iter().enumerate() {

            let path = format!("archive/assets/images/p{}_{}.jpg",i,j);

            download(img,&path);

            md.push_str(&format!("![](/{} )\n\n",path));

            images_total += 1;

        }

        for (j,vid) in p.videos.iter().enumerate() {

            let path = format!("archive/assets/videos/p{}_{}.mp4",i,j);

            download(vid,&path);

            md.push_str(&format!(
"<video controls src=\"/{}\" width=\"600\"></video>\n\n",
path
));

            videos_total += 1;

        }

    }

    let id = Utc::now().timestamp().to_string();

    let file = format!("archive/posts/{}.md",id);

    fs::write(&file,md).unwrap();

    let meta_path = "archive/meta.json";

    let mut meta = load_meta(meta_path);

    meta.push(Entry{
        id: id.clone(),
        title: format!("Archive {}",id),
        r#type: "web".into(),
        url: "".into(),
        date: Utc::now(),
        file: file.clone(),
        stats: Stats{
            posts: posts.len(),
            images: images_total,
            videos: videos_total,
        }
    });

    save_meta(meta_path,&meta);

    build_index(&meta);

}
