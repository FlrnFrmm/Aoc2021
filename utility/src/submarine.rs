#[derive(Debug)]
pub enum SubmarineCommand {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl SubmarineCommand {
    pub fn from(input: (&str, i32)) -> anyhow::Result<SubmarineCommand> {
        use SubmarineCommand::*;

        let command = match input {
            ("forward", v) => Forward(v),
            ("up", v) => Up(v),
            ("down", v) => Down(v),
            (cmd, _) => anyhow::bail!("Unknow submarine command: {}", cmd),
        };

        Ok(command)
    }
}
