use std::io::Write;
use std::path::Path;
use std::{env, fs};

use gray_matter::{engine, Matter};

include!("src/tags.rs");

/// generate the ChoiceParameter enum and tag data we will use in the `tag` command
#[allow(dead_code)]
fn main() {
	let out_dir = env::var_os("OUT_DIR").unwrap();
	let dest_file = Path::new(&out_dir).join("generated.rs");
	let mut file = fs::File::create(dest_file).unwrap();

	let tag_files: Vec<String> = fs::read_dir(TAG_DIR)
		.unwrap()
		.map(|f| f.unwrap().file_name().to_string_lossy().to_string())
		.collect();

	let mut tags: Vec<Tag> = tag_files
        .clone()
        .into_iter()
        .map(|name| {
            let file_name = format!("{TAG_DIR}/{name}");
            let file_content = fs::read_to_string(&file_name).unwrap();

            let matter = Matter::<engine::YAML>::new();
            let parsed = matter.parse(&file_content);
            let content = parsed.content;
            let data = parsed
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
                id: name.trim_end_matches(".md").to_string(),
                frontmatter: data,
            }
        })
        .collect();

	tags.sort_by(|t1, t2| t1.id.cmp(&t2.id));

	let tag_names: Vec<String> = tags.iter().map(|t| format!("{},", t.id)).collect();

	let tag_matches: Vec<String> = tags
		.iter()
		.map(|t| format!("Self::{} => \"{}\",", t.id, t.id))
		.collect();

	writeln!(
		file,
		r#"#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Clone, Debug, poise::ChoiceParameter)]
pub enum Choice {{
    {}
}}"#,
		tag_names.join("\n")
	)
	.unwrap();

	writeln!(
		file,
		r#"impl Choice {{
  fn as_str(&self) -> &str {{
    match &self {{
      {}
    }}
  }}
}}"#,
		tag_matches.join("\n")
	)
	.unwrap();

	println!(
		"cargo:rustc-env=TAGS={}",
		// make sure we can deserialize with env! at runtime
		serde_json::to_string(&tags).unwrap()
	);
}
