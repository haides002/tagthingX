use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct File {
    pub path: std::path::PathBuf,
    pub tags: Option<Vec<String>>,
    pub date: Option<chrono::NaiveDateTime>,
    pub thumbnail_path: Option<std::path::PathBuf>,
}

impl File {
    pub fn new(path: std::path::PathBuf) -> Self {
        use allmytoes::*;
        use exempi2::{OpenFlags, PropFlags, Xmp, XmpFile, XmpString};
        use std::path::PathBuf;

        const THUMBNAIL_SIZE: ThumbSize = ThumbSize::Large;

        const EXIF_SCHEMA: &str = "http://ns.adobe.com/exif/1.0/";
        const DUBLIN_CORE_SCHEMA: &str = "http://purl.org/dc/elements/1.1/";

        println!("reading file {}", path.to_str().unwrap());

        let xmp: Option<Xmp> = XmpFile::new_from_file(&path, OpenFlags::ONLY_XMP)
            .expect("failed to read file")
            .get_new_xmp()
            .ok();

        let tags = || -> Option<Vec<String>> {
            let mut tags: Vec<String> = Vec::new();

            for i in 1.. {
                match xmp.clone()?.get_array_item(
                    DUBLIN_CORE_SCHEMA,
                    "dc:subject",
                    i,
                    &mut PropFlags::empty(),
                ) {
                    Ok(tag) => tags.push(tag.to_string()),
                    Err(_) => break,
                }
            }

            Some(tags)
        }();

        let date = || -> Option<NaiveDateTime> {
            let formats: Vec<&str> = vec!["%+", "%FT%T", "%FT%T%.f"];

            let exif_date: Result<XmpString, exempi2::Error> =
                xmp.clone()?
                    .get_property(EXIF_SCHEMA, "DateTimeOriginal", &mut PropFlags::empty());

            let date = exif_date.ok()?;

            match formats
                .iter()
                .map(|format: &&str| -> Result<NaiveDateTime, _> {
                    chrono::NaiveDateTime::parse_from_str(date.to_str().unwrap(), *format)
                })
                .filter(|result| -> bool { result.is_ok() })
                .nth(0)
            {
                Some(date) => Some(date.unwrap()),
                None => panic!("unknown date format for date: {}", date.to_str().unwrap()),
            }
        }();

        let thumbnail_path = || -> Option<PathBuf> {
            let config = AMTConfiguration {
                force_creation: false,
                return_smallest_feasible: false,
                leak: false,
                custom_provider_spec_file: None,
                force_inbuilt_provider_spec: false,
            };

            Some(PathBuf::from(
                AMT::new(&config).get(&path, THUMBNAIL_SIZE).ok()?.path,
            ))
        }();

        File {
            path,
            tags,
            date,
            thumbnail_path,
        }
    }

    pub fn read_directory(path: std::path::PathBuf) -> Vec<Self> {
        use std::path::PathBuf;
        let mut files: Vec<Self> = Vec::new();

        for entry in std::fs::read_dir(path).expect("failed to read directory") {
            let path: PathBuf = entry.unwrap().path();

            if path.is_dir() {
                files.append(&mut Self::read_directory(path));
            } else if path.is_file() {
                files.push(Self::new(path));
            } else {
                // its likely a symlink these are not handled yet
                todo!();
            }
        }

        files
    }
}
