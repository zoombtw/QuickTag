use std::{path::Path, collections::HashMap};

use serde_derive::Deserialize;
use toml::value::Table;
use audiotags::{Tag, Picture, MimeType};

#[derive(Deserialize)]
struct Config {
    artist: String,
    move_into_folder: bool,
    delete_original: bool,
    albums: Table
}

#[derive(Deserialize)]
struct Album {
    title: String,
    folder: String,
    cover: String
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    return input;
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("Usage: {} <files>...", args[0]);
        return;
    }

    let raw_config = std::fs::read_to_string("config.toml").unwrap();
    let config: Config = toml::from_str(&raw_config).unwrap();

    let mut raw_covers = HashMap::new();
    for (_, album) in config.albums.iter() {
        let album: Album = album.clone().try_into().unwrap();
        let raw_cover = std::fs::read(format!("covers/{}", album.cover));

        if let Ok(raw_cover) = raw_cover {
            raw_covers.insert(album.title, raw_cover);
        }
        else {
            println!("Could not read cover for album '{}'", album.title);
        }
    }

    for path in args[1..].iter() {
        println!("Processing {}", path);
        let mut tag = if let Ok(tag) = Tag::default().read_from_path(path) {
            tag
        }
        else {
            println!("Could not read file {}", path);
            continue;
        };

        tag.set_artist(&config.artist);

        println!("Please input the song title:");
        let title = read_line();
        tag.set_title(&title.trim());

        println!("Albums:");
        for (_i, (title, _)) in config.albums.iter().enumerate() {
            println!("{}", title);
        }

        println!("Please input the album name:");
        let album: Album = config.albums.get(read_line().trim()).unwrap().clone().try_into().unwrap();

        tag.set_album(audiotags::Album{
            title: &album.title,
            artist: Some(&config.artist),
            cover: Some(Picture {
                data: &raw_covers[&album.title],
                mime_type: match Path::new(&album.cover).extension().unwrap().to_str().unwrap() {
                    "jpg" => MimeType::Jpeg,
                    "png" => MimeType::Png,
                    _ => panic!("Unsupported cover format")
                }
            })
        });

        let extension = Path::new(path).extension().unwrap().to_str().unwrap();
        let new_path = if config.move_into_folder {
            if Path::new(&album.folder).exists() {
                std::fs::create_dir_all(&album.folder).unwrap();
            }

            format!("{}/{}.{}", album.folder, title.trim(), extension)
        } else {
            format!("{}.{}", title.trim(), extension)
        };

        std::fs::copy(path, &new_path).unwrap();
        tag.write_to_path(&new_path).unwrap();
        
        if config.delete_original {
            std::fs::remove_file(path).unwrap();
        }
    }
}
