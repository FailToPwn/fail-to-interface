pub trait PluginService {
    fn test_log_is_parsable(&self, log_to_parse: &str) -> bool;

    fn parse_log(&self, log_to_parse: &str);
}
