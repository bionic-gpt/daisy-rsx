#![allow(non_snake_case)]
use std::fmt::Display;

use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum FileInputStyle {
    #[default]
    Default,
    Ghost,
}

impl Display for FileInputStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileInputStyle::Default => write!(f, ""),
            FileInputStyle::Ghost => write!(f, "file-input-ghost"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum FileInputColor {
    #[default]
    Default,
    Neutral,
    Primary,
    Secondary,
    Accent,
    Info,
    Success,
    Warning,
    Error,
}

impl Display for FileInputColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileInputColor::Default => write!(f, ""),
            FileInputColor::Neutral => write!(f, "file-input-neutral"),
            FileInputColor::Primary => write!(f, "file-input-primary"),
            FileInputColor::Secondary => write!(f, "file-input-secondary"),
            FileInputColor::Accent => write!(f, "file-input-accent"),
            FileInputColor::Info => write!(f, "file-input-info"),
            FileInputColor::Success => write!(f, "file-input-success"),
            FileInputColor::Warning => write!(f, "file-input-warning"),
            FileInputColor::Error => write!(f, "file-input-error"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum FileInputSize {
    #[default]
    Md,
    Xs,
    Sm,
    Lg,
    Xl,
}

impl Display for FileInputSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileInputSize::Md => write!(f, "file-input-md"),
            FileInputSize::Xs => write!(f, "file-input-xs"),
            FileInputSize::Sm => write!(f, "file-input-sm"),
            FileInputSize::Lg => write!(f, "file-input-lg"),
            FileInputSize::Xl => write!(f, "file-input-xl"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct FileInputProps {
    class: Option<String>,
    id: Option<String>,
    name: Option<String>,
    accept: Option<String>,
    multiple: Option<bool>,
    required: Option<bool>,
    disabled: Option<bool>,
    file_input_style: Option<FileInputStyle>,
    file_input_color: Option<FileInputColor>,
    file_input_size: Option<FileInputSize>,
}

#[component]
pub fn FileInput(props: FileInputProps) -> Element {
    let style = props.file_input_style.unwrap_or_default();
    let color = props.file_input_color.unwrap_or_default();
    let size = props.file_input_size.unwrap_or_default();
    let class = props.class.unwrap_or_default();
    let disabled = props.disabled.filter(|&d| d);

    rsx!(
        input {
            "type": "file",
            id: props.id,
            name: props.name,
            accept: props.accept,
            multiple: props.multiple,
            required: props.required,
            disabled,
            class: "file-input {class} {style} {color} {size}",
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_input() {
        let props = FileInputProps {
            class: Some("custom".to_string()),
            id: Some("id".to_string()),
            name: Some("name".to_string()),
            accept: Some("image/*".to_string()),
            multiple: Some(true),
            required: Some(true),
            disabled: Some(false),
            file_input_style: Some(FileInputStyle::Ghost),
            file_input_color: Some(FileInputColor::Primary),
            file_input_size: Some(FileInputSize::Lg),
        };

        let result = dioxus_ssr::render_element(FileInput(props));
        assert!(result.contains("file-input-ghost"));
        assert!(result.contains("file-input-primary"));
        assert!(result.contains("file-input-lg"));
        assert!(result.contains("class=\"file-input custom"));
        assert!(result.contains("required=true"));
    }

    #[test]
    fn test_file_input_default() {
        let props = FileInputProps {
            class: None,
            id: None,
            name: None,
            accept: None,
            multiple: None,
            required: None,
            disabled: None,
            file_input_style: None,
            file_input_color: None,
            file_input_size: None,
        };

        let result = dioxus_ssr::render_element(FileInput(props));
        assert!(result.contains("file-input-md"));
    }
}
