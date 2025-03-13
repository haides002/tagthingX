#[derive(Debug)]
pub struct File {
    path: std::path::PathBuf,
    tags: Vec<String>,
    date: Option<chrono::DateTime<chrono::FixedOffset>>,
    thumbnail_path: std::path::PathBuf,
}

impl File {
    pub fn new(path: std::path::PathBuf) -> Self {
        use allmytoes::*;
        use chrono::{DateTime, FixedOffset};
        use exempi2::{OpenFlags, PropFlags, Xmp, XmpFile};
        use std::path::PathBuf;

        const THUMBNAIL_SIZE: ThumbSize = ThumbSize::Large;
        const EXIF_SCHEMA: &str = "http://ns.adobe.com/exif/1.0/";
        const XMP_SCHEMA: &str = "http://ns.adobe.com/xap/1.0/";
        const DUBLIN_CORE_SCHEMA: &str = "http://purl.org/dc/elements/1.1/";

        let xmp: Xmp = XmpFile::new_from_file(&path, OpenFlags::ONLY_XMP)
            .expect("failed to read file")
            .get_new_xmp()
            .expect("failed to read XMP");

        let tags: Vec<String> = {
            let mut tags: Vec<String> = Vec::new();

            for i in 1.. {
                match xmp.get_array_item(
                    DUBLIN_CORE_SCHEMA,
                    "dc:subject",
                    i,
                    &mut PropFlags::empty(),
                ) {
                    Ok(tag) => tags.push(tag.to_string()),
                    Err(_) => break,
                }
            }

            tags
        };

        let date: Option<DateTime<FixedOffset>> = {
            let exif_date =
                xmp.get_property(EXIF_SCHEMA, "DateTimeOriginal", &mut PropFlags::empty());

            match exif_date {
                Ok(date) => Some(
                    DateTime::parse_from_str(date.to_str().unwrap(), "%+")
                        .expect("could not parse date"),
                ),
                Err(_) => None,
            }
        };

        let thumbnail_path: PathBuf = {
            let config: AMTConfiguration = AMTConfiguration {
                force_creation: false,
                return_smallest_feasible: false,
                leak: false,
                custom_provider_spec_file: None,
                force_inbuilt_provider_spec: false,
            };

            PathBuf::from(
                AMT::new(&config)
                    .get(&path, THUMBNAIL_SIZE)
                    .expect("failed to generate thumbnail")
                    .path,
            )
        };

        File {
            path,
            tags,
            date,
            thumbnail_path,
        }
    }
}
