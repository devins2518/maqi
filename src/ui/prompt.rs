pub(super) struct Prompt<'a> {
    msg: &'a str,
    response: String,
}

impl<'a> Prompt<'a> {
    pub fn new(msg: &'a str) -> Self {
        Prompt {
            msg,
            response: String::new(),
        }
    }

    pub fn run(self) -> String {
        unimplemented!();
        return self.response;
    }
}
