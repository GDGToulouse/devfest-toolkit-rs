use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;

use anyhow::Result;
use serde::Serialize;

use dftk_common::models::Markdown;

pub trait FrontMatterMarkdown<T>
where
    T: Serialize,
{
    fn unique_key(&self) -> String;

    fn front_matter(&self) -> T;

    fn content(&self) -> Markdown;

    fn write_to_dir(&self, path: &PathBuf) -> Result<()> {
        let key = self.unique_key();
        let front_matter = self.front_matter();
        let content: String = self.content().into();

        let mut file_path = path.clone();
        file_path.push(format!("{}.md", key));

        debug!("  write {} to {:?}", key, file_path);
        let mut file = File::create(file_path)?;
        let fm = serde_yaml::to_string(&front_matter)?
            .lines()
            .filter(|&line| !line.starts_with("---"))
            // .filter(|&line| !line.contains("~"))
            .collect::<Vec<&str>>()
            .join("\n");
        file.write_fmt(format_args!("{}\n---\n\n{}", fm, content))?;

        Ok(())
    }
}

pub struct FrontMatterMarkdownWriter {
    parent_path: PathBuf,
    label: String,
}

impl FrontMatterMarkdownWriter {
    pub fn new(label: &str, parent_path: PathBuf) -> Self {
        Self {
            label: label.into(),
            parent_path,
        }
    }

    pub fn write_all<T, S>(&self, elements: &[T]) -> Result<usize>
    where
        S: Serialize,
        T: FrontMatterMarkdown<S> + Sized,
    {
        create_dir_all(self.parent_path.clone())?;
        info!("Write all {} to {:?}", self.label, self.parent_path);

        for element in elements {
            element.write_to_dir(&self.parent_path)?;
        }

        Ok(elements.len())
    }
}
