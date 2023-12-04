use std::collections::HashMap;
use std::path::Path;
use std::{env, fs};

use gray_matter::{engine, Matter};

include!("src/tags.rs");

/// generate the ChoiceParameter enum and tag data we will use in the `tag` command
#[allow(dead_code)]
fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let generated = Path::new(&out_dir).join("generated.rs");

    let tag_files: Vec<String> = fs::read_dir(TAG_DIR)
        .unwrap()
        .map(|f| f.unwrap().file_name().to_string_lossy().to_string())
        .collect();

    let tags: Vec<Tag> = tag_files
        .clone()
        .into_iter()
        .map(|name| {
            let file_name = format!("{TAG_DIR}/{name}");
            let content = fs::read_to_string(&file_name).unwrap();

            let matter = Matter::<engine::YAML>::new();
            let frontmatter: TagFrontmatter = matter
                .parse(&content)
                .data
                .unwrap()
                .deserialize()
                .unwrap_or_else(|e| {
                    // actually handling the error since this is the most likely thing to fail -getchoo
                    panic!(
                        "Failed to parse file {file_name}! Here's what it looked like:\n{content}\n\nReported Error:\n{e}\n",
                    )
                });

            Tag {
                content,
                file_name: name,
                frontmatter,
            }
        })
        .collect();

    let aliases: HashMap<String, Vec<String>> = tags
        .iter()
        .filter_map(|t| {
            t.frontmatter
                .aliases
                .clone()
                .map(|aliases| (t.file_name.clone(), aliases))
        })
        .collect();

    let formatted_names: Vec<String> = tags
        .iter()
        .flat_map(|t| {
            let mut res = Vec::from([t.file_name.replace(".md", "").replace('-', "_")]);
            if let Some(tag_aliases) = aliases.get(&t.file_name) {
                res.append(&mut tag_aliases.clone())
            }

            res
        })
        .collect();

    let tag_choice = format!(
        r#"
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    #[derive(Clone, Debug, poise::ChoiceParameter)]
    pub enum TagChoice {{
    {}
    }}"#,
        formatted_names.join(",\n")
    );

    let to_str = format!(
        r#"
    impl TagChoice {{
    fn as_str(&self) -> &str {{
    match &self {{
    {}
    }}
    }}
    }}
    "#,
        formatted_names
            .iter()
            .map(|n| {
                let file_name = n.replace('_', "-") + ".md";

                // assume this is an alias if we can't match the file name
                let name = if tag_files.contains(&file_name) {
                    file_name
                } else {
                    aliases
                        .iter()
                        .find(|a| a.1.contains(n))
                        .unwrap()
                        .0
                        .to_string()
                };

                format!("Self::{n} => \"{name}\",")
            })
            .collect::<Vec<String>>()
            .join("\n")
    );

    let contents = Vec::from([tag_choice, to_str]).join("\n\n");

    fs::write(generated, contents).unwrap();
    println!(
        "cargo:rustc-env=TAGS={}",
        // make sure we can deserialize with env! at runtime
        serde_json::to_string(&tags).unwrap()
    );
}
