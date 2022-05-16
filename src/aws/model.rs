use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Builder, PartialEq, PartialOrd, Debug, Clone)]
pub struct LogQueryInfo {
    pub id: String,
    pub name: String,
    pub query: String,
    pub log_group_names: Vec<String>,
}

impl LogQueryInfo {
    pub fn to_string(&self, full: bool) -> String {
        if full {
            format!(
                "{} ({}) ({})-> \n{}\n",
                self.name,
                self.id,
                self.log_group_names.join(", "),
                self.query
            )
        } else {
            format!(
                "{} ({}) ({})\n",
                self.name,
                self.id,
                self.log_group_names.join(", "),
            )
        }
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct LogQueryInfoList {
    pub queries: Vec<LogQueryInfo>,
}

impl LogQueryInfoList {
    pub fn to_string(&self, full: bool) -> String {
        let data = self
            .queries
            .iter()
            .map(|query| format!("{}\n", query.to_string(full)))
            .collect::<String>();

        format!("{}", data)
    }

    pub fn find(&self, query_id: String) -> Option<LogQueryInfo> {
        self.queries
            .iter()
            .find(|query| query.id == query_id)
            .map(LogQueryInfo::clone)
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct LogResults {
    pub lines: Vec<LogLine>,
}

impl LogResults {
    pub fn to_string(&self, fields: Option<Vec<String>>) -> String {
        self.lines
            .iter()
            .map(|line| format!("{}\n", line.to_string(&fields)))
            .collect::<String>()
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct LogLine {
    pub fields: Vec<LogField>,
}

impl LogLine {
    pub fn to_string(&self, fields: &Option<Vec<String>>) -> String {
        self.fields
            .iter()
            .filter(|field| match &fields {
                Some(_) => fields.as_ref().unwrap().contains(&field.field),
                None => true,
            })
            .map(|field| format!("{}\n", field))
            .collect::<String>()
    }
}

#[derive(Builder, PartialEq, PartialOrd, Debug)]
pub struct LogField {
    pub field: String,
    pub value: String,
}

impl Display for LogField {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}: {}", self.field, self.value)
    }
}

#[derive(Debug)]
pub struct TerminalError {
    details: String,
}

impl TerminalError {
    pub fn new(msg: &str) -> Self {
        TerminalError {
            details: msg.to_string(),
        }
    }
}

impl Display for TerminalError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.details)
    }
}

impl Error for TerminalError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_format_log_query_info_with_log_group() {
        let log_query_info = LogQueryInfoBuilder::default()
            .id("dinosaur".to_string())
            .name("DinoQuery".to_string())
            .query("fields dinosaur".to_string())
            .log_group_names(vec![
                "dinosaur::log".to_string(),
                "dinosaur::log2".to_string(),
            ])
            .build()
            .unwrap();

        assert_eq!(
            "DinoQuery (dinosaur) (dinosaur::log, dinosaur::log2)-> \nfields dinosaur\n",
            format!("{}", log_query_info.to_string(true))
        );

        assert_eq!(
            "DinoQuery (dinosaur) (dinosaur::log, dinosaur::log2)\n",
            format!("{}", log_query_info.to_string(false))
        );
    }

    #[test]
    fn should_format_log_query_info_with_not_log_group() {
        let log_query_info = LogQueryInfoBuilder::default()
            .id("dinosaur".to_string())
            .name("DinoQuery".to_string())
            .query("fields dinosaur".to_string())
            .log_group_names(vec![])
            .build()
            .unwrap();

        assert_eq!(
            "DinoQuery (dinosaur) ()-> \nfields dinosaur\n",
            format!("{}", log_query_info.to_string(true))
        );

        assert_eq!(
            "DinoQuery (dinosaur) ()\n",
            format!("{}", log_query_info.to_string(false))
        );
    }

    #[test]
    fn should_format_log_query_info_list() {
        let log_query_info_list = LogQueryInfoList {
            queries: vec![
                LogQueryInfoBuilder::default()
                    .id("dinosaur".to_string())
                    .name("DinoQuery".to_string())
                    .query("fields dinosaur".to_string())
                    .log_group_names(vec!["dinosaur::log".to_string()])
                    .build()
                    .unwrap(),
                LogQueryInfoBuilder::default()
                    .id("dinosaur".to_string())
                    .name("DinoQuery2".to_string())
                    .query("fields dinosaur".to_string())
                    .log_group_names(vec![])
                    .build()
                    .unwrap(),
            ],
        };

        assert_eq!("DinoQuery (dinosaur) (dinosaur::log)-> \nfields dinosaur\n\nDinoQuery2 (dinosaur) ()-> \nfields dinosaur\n\n", format!("{}", log_query_info_list.to_string(true)));
        assert_eq!(
            "DinoQuery (dinosaur) (dinosaur::log)\n\nDinoQuery2 (dinosaur) ()\n\n",
            format!("{}", log_query_info_list.to_string(false))
        );
    }

    #[test]
    fn should_return_query_info_when_found() {
        let expected_query = LogQueryInfoBuilder::default()
            .id("dinosaur".to_string())
            .name("DinoQuery".to_string())
            .query("fields dinosaur".to_string())
            .log_group_names(vec!["dinosaur::log".to_string()])
            .build()
            .unwrap();

        let log_query_info_list = LogQueryInfoList {
            queries: vec![
                LogQueryInfoBuilder::default()
                    .id("dinosaur".to_string())
                    .name("DinoQuery".to_string())
                    .query("fields dinosaur".to_string())
                    .log_group_names(vec!["dinosaur::log".to_string()])
                    .build()
                    .unwrap(),
                LogQueryInfoBuilder::default()
                    .id("dinosaur2".to_string())
                    .name("DinoQuery2".to_string())
                    .query("fields dinosaur".to_string())
                    .log_group_names(vec![])
                    .build()
                    .unwrap(),
            ],
        };

        let found = log_query_info_list.find("dinosaur".to_string());
        assert!(found.is_some());
        assert_eq!(found.unwrap(), expected_query);
    }

    #[test]
    fn should_return_empty_when_query_info_not_when_found() {
        let log_query_info_list = LogQueryInfoList {
            queries: vec![
                LogQueryInfoBuilder::default()
                    .id("dinosaur".to_string())
                    .name("DinoQuery".to_string())
                    .query("fields dinosaur".to_string())
                    .log_group_names(vec!["dinosaur::log".to_string()])
                    .build()
                    .unwrap(),
                LogQueryInfoBuilder::default()
                    .id("dinosaur2".to_string())
                    .name("DinoQuery2".to_string())
                    .query("fields dinosaur".to_string())
                    .log_group_names(vec![])
                    .build()
                    .unwrap(),
            ],
        };

        let found = log_query_info_list.find("batata".to_string());
        assert!(found.is_none());
    }

    #[test]
    fn terminal_error_should_display_message() {
        let error = TerminalError::new("batata");

        assert_eq!("batata", format!("{}", error));
    }

    #[test]
    fn log_field_should_display_message() {
        let log_field = LogField {
            field: "@message".to_string(),
            value: "Batata".to_string(),
        };

        assert_eq!(format!("{}", log_field), "@message: Batata")
    }

    #[test]
    fn log_line_should_display_all_fields() {
        let log_line = LogLine {
            fields: vec![
                LogField {
                    field: "@timestamp".to_string(),
                    value: "2022-05-15 19:37:19.581".to_string(),
                },
                LogField {
                    field: "@message".to_string(),
                    value: "Batata".to_string(),
                },
            ],
        };

        assert_eq!(
            format!("{}", log_line.to_string(&None)),
            "@timestamp: 2022-05-15 19:37:19.581\n@message: Batata\n"
        )
    }

    #[test]
    fn log_line_should_display_only_provided_fields() {
        let log_line = LogLine {
            fields: vec![
                LogField {
                    field: "@timestamp".to_string(),
                    value: "2022-05-15 19:37:19.581".to_string(),
                },
                LogField {
                    field: "@message".to_string(),
                    value: "Batata".to_string(),
                },
            ],
        };

        assert_eq!(
            format!(
                "{}",
                log_line.to_string(&Some(vec!["@message".to_string()]))
            ),
            "@message: Batata\n"
        )
    }

    #[test]
    fn log_results_should_display_all_log_lines() {
        let log_results = LogResults {
            lines: vec![
                LogLine {
                    fields: vec![
                        LogField {
                            field: "@timestamp".to_string(),
                            value: "2022-05-15 19:37:19.581".to_string(),
                        },
                        LogField {
                            field: "@message".to_string(),
                            value: "Batata".to_string(),
                        },
                    ],
                },
                LogLine {
                    fields: vec![
                        LogField {
                            field: "@timestamp".to_string(),
                            value: "2022-04-15 19:37:19.581".to_string(),
                        },
                        LogField {
                            field: "@message".to_string(),
                            value: "Frita".to_string(),
                        },
                    ],
                },
            ],
        };

        assert_eq!(format!("{}", log_results.to_string(None)), "@timestamp: 2022-05-15 19:37:19.581\n@message: Batata\n\n@timestamp: 2022-04-15 19:37:19.581\n@message: Frita\n\n");
    }

    #[test]
    fn log_results_should_display_all_log_lines_with_provided_fields() {
        let log_results = LogResults {
            lines: vec![
                LogLine {
                    fields: vec![
                        LogField {
                            field: "@timestamp".to_string(),
                            value: "2022-05-15 19:37:19.581".to_string(),
                        },
                        LogField {
                            field: "@message".to_string(),
                            value: "Batata".to_string(),
                        },
                    ],
                },
                LogLine {
                    fields: vec![
                        LogField {
                            field: "@timestamp".to_string(),
                            value: "2022-04-15 19:37:19.581".to_string(),
                        },
                        LogField {
                            field: "@message".to_string(),
                            value: "Frita".to_string(),
                        },
                    ],
                },
            ],
        };

        assert_eq!(
            format!(
                "{}",
                log_results.to_string(Some(vec!["@message".to_string()]))
            ),
            "@message: Batata\n\n@message: Frita\n\n"
        );
    }
}
