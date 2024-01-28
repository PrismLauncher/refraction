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
                file_name: name,
                frontmatter: data,
            }
        })
        .collect();

	let formatted_names: Vec<String> = tags
		.iter()
		.map(|t| t.file_name.replace(".md", "").replace('-', "_"))
		.collect();

	let tag_choice = format!(
		r#"
    #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
    #[derive(Clone, Debug, poise::ChoiceParameter)]
    pub enum Choice {{
    {}
    }}"#,
		formatted_names.join(",\n")
	);

	let to_str = format!(
		r#"
    impl Choice {{
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
				format!("Self::{n} => \"{file_name}\",")
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
