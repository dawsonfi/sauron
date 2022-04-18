use std::fmt::{Display, Formatter, Result};

#[derive(Builder, PartialEq, PartialOrd, Debug)]
pub struct LogQueryInfo {
    pub query_id: String,
    pub query_string: String,
    pub log_group_names: Vec<String>,
}

impl Display for LogQueryInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Query Id: {} ({})-> \n{}\n",
            self.query_id,
            self.log_group_names.join(", "),
            self.query_string
        )
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct LogQueryInfoList {
    pub queries: Vec<LogQueryInfo>,
}

impl Display for LogQueryInfoList {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let data = self
            .queries
            .iter()
            .map(|query| format!("{}\n", query))
            .collect::<String>();

        write!(f, "{}", data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_format_log_query_info_with_log_group() {
        let log_query_info = LogQueryInfoBuilder::default()
            .query_id("dinosaur".to_string())
            .query_string("fields dinosaur".to_string())
            .log_group_names(vec!["dinosaur::log".to_string(), "dinosaur::log2".to_string()])
            .build()
            .unwrap();

        assert_eq!(
            "Query Id: dinosaur (dinosaur::log, dinosaur::log2)-> \nfields dinosaur\n",
            format!("{}", log_query_info)
        );
    }

    #[test]
    fn should_format_log_query_info_with_not_log_group() {
        let log_query_info = LogQueryInfoBuilder::default()
            .query_id("dinosaur".to_string())
            .query_string("fields dinosaur".to_string())
            .log_group_names(vec![])
            .build()
            .unwrap();

        assert_eq!(
            "Query Id: dinosaur ()-> \nfields dinosaur\n",
            format!("{}", log_query_info)
        );
    }

    #[test]
    fn should_format_log_query_info_list() {
        let log_query_info_list = LogQueryInfoList {
            queries: vec![
                LogQueryInfoBuilder::default()
                    .query_id("dinosaur".to_string())
                    .query_string("fields dinosaur".to_string())
                    .log_group_names(vec!["dinosaur::log".to_string()])
                    .build()
                    .unwrap(),
                LogQueryInfoBuilder::default()
                    .query_id("dinosaur".to_string())
                    .query_string("fields dinosaur".to_string())
                    .log_group_names(vec![])
                    .build()
                    .unwrap(),
            ],
        };

        assert_eq!("Query Id: dinosaur (dinosaur::log)-> \nfields dinosaur\n\nQuery Id: dinosaur ()-> \nfields dinosaur\n\n", format!("{}", log_query_info_list));
    }
}
