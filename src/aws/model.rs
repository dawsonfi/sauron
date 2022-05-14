#[derive(Builder, PartialEq, PartialOrd, Debug)]
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
}

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
}
