#[derive(Builder, PartialEq, PartialOrd, Debug)]
pub struct LogQueryInfo {
    pub query_id: String,
    pub query_string: String,
    pub log_group_name: Option<String>,
}
