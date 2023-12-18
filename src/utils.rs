pub fn title_to_id(title: &str) -> String {
    title.to_lowercase().replace(' ', "-")
}
