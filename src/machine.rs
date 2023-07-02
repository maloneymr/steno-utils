use super::plover_dict::Token;


pub struct Machine {
    history: Vec<Token>,
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            history: vec![],
        }
    }

    pub fn input(&mut self, token: Token) {
        self.history.push(token);
    }

    pub fn commit(&mut self) -> String {
        self.history = vec![];
        "".to_string()
    }
}
