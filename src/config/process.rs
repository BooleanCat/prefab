#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Process {
    #[serde(default)]
    pub terminal: bool,

    #[serde(default)]
    pub console_size: ConsoleSize,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct ConsoleSize {
    pub height: usize,
    pub width: usize,
}

#[cfg(test)]
mod tests {
    use super::Process;
    use serde_json;

    #[test]
    fn deserialize_process_terminal_optional() {
        let process: Process = serde_json::from_str("{}").unwrap();
        let expected = Process{terminal: false, ..Default::default()};

        assert_eq!(expected, process);
    }

    #[test]
    fn deserialize_process_console_size_optional() {
        let process: Process = serde_json::from_str("{}").unwrap();
        let expected = Process{console_size: Default::default(), ..Default::default()};

        assert_eq!(expected, process);
    }
}
