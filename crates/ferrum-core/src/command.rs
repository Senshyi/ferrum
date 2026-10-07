#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Set { key: String, value: String },
    Get { key: String },
    Del { key: String },
    Keys { prefix: String },
}

pub fn parse(line: &str) -> Option<Command> {
    if line.is_empty() {
        return None;
    }
    let mut words = line.split_whitespace();
    let command = words.next()?;
    let key = words.next()?.to_string();

    match command.to_lowercase().as_str() {
        "get" => Some(Command::Get { key }),
        "set" => Some(Command::Set {
            key,
            value: words.next()?.to_string(),
        }),
        "del" => Some(Command::Del { key }),
        "keys" => Some(Command::Keys { prefix: key }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_get_commant() {
        assert_eq!(
            parse("get key1").unwrap(),
            Command::Get {
                key: String::from("key1")
            }
        )
    }

    #[test]
    fn parse_set_commant() {
        assert_eq!(
            parse("set key1 value1").unwrap(),
            Command::Set {
                key: String::from("key1"),
                value: String::from("value1")
            }
        )
    }

    #[test]
    fn parse_del_commant() {
        assert_eq!(
            parse("del key1").unwrap(),
            Command::Del {
                key: String::from("key1")
            }
        )
    }
    #[test]
    fn parse_keys_commant() {
        assert_eq!(
            parse("keys ke").unwrap(),
            Command::Keys {
                prefix: String::from("ke")
            }
        )
    }
    #[test]
    fn parse_command_is_case_insensitive() {
        assert_eq!(
            parse("GET key1").unwrap(),
            Command::Get {
                key: String::from("key1")
            }
        );
        assert_eq!(
            parse("sEt key1 val").unwrap(),
            Command::Set {
                key: String::from("key1"),
                value: String::from("val")
            }
        );
        assert_eq!(
            parse("DeL kEy1").unwrap(),
            Command::Del {
                key: String::from("kEy1")
            }
        )
    }

    #[test]
    fn parse_unkown_command() {
        assert_eq!(parse("hello world"), None)
    }
    #[test]
    fn parse_extra_args() {
        assert_eq!(
            parse("get hello world").unwrap(),
            Command::Get {
                key: String::from("hello")
            }
        )
    }
}
