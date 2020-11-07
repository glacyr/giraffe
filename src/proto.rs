pub enum Multiplexed<T> {
    Pack(T),
    Progress(String),
    Fatal(String),
}

pub struct Status<E> {
    pub unpack_error: Option<E>,
    pub commands: Vec<CommandStatus>,
}

pub enum CommandStatus {
    Ok(String),
    NoGood(String, String),
}

pub fn packet(source: impl AsRef<str>) -> String {
    let length = format!("{:04x}", source.as_ref().len() + 4);
    length + source.as_ref()
}
