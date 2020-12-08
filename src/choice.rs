use crate::Prompt;

#[derive(Copy, Clone)]
pub struct ChoiceMachine<'inner_choice> {
    a: Option<Choice<'inner_choice>>,
    b: Option<Choice<'inner_choice>>,
    c: Option<Choice<'inner_choice>>,
    d: Option<Choice<'inner_choice>>,
}

impl<'inner_choice> ChoiceMachine<'inner_choice> {
    pub fn new(choices: &'inner_choice [Choice<'inner_choice>]) -> Self {
        let mut choices_iter = choices.iter();
        Self {
            a: choices_iter.next().copied(),
            b: choices_iter.next().copied(),
            c: choices_iter.next().copied(),
            d: choices_iter.next().copied(),
        }
    }

    pub fn a(&self) -> Option<Choice<'inner_choice>> {
        self.a
    }
    pub fn b(&self) -> Option<Choice<'inner_choice>> {
        self.b
    }
    pub fn c(&self) -> Option<Choice<'inner_choice>> {
        self.c
    }
    pub fn d(&self) -> Option<Choice<'inner_choice>> {
        self.d
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Choice<'prompt> {
    A(&'prompt str),
    B(&'prompt str),
    C(&'prompt str),
    D(&'prompt str),
}

impl<'inner> std::fmt::Display for Choice<'inner> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Choice::A(v) => {
                write!(f, "A - {}", v)?;
            }
            Choice::B(v) => {
                write!(f, "B - {}", v)?;
            }
            Choice::C(v) => {
                write!(f, "C - {}", v)?;
            }
            Choice::D(v) => {
                write!(f, "D - {}", v)?;
            }
        }
        Ok(())
    }
}

impl<'choice_machine> ChoiceMachine<'choice_machine> {
    pub fn exec(&self) -> Choice<'choice_machine> {
        &[self.a(), self.b(), self.c(), self.d()]
            .iter()
            .filter(|e| e.is_some())
            .for_each(|choice| println!("{}", choice.unwrap()));
        let input = Prompt::new()
            .set_hidden(false)
            .set_max_length(1)
            .execute()
            .unwrap();
        match input.chars().nth(0).unwrap() {
            'a' => return self.a().unwrap(),
            'b' => return self.b.unwrap(),
            'c' => return self.c().unwrap(),
            'd' => return self.d().unwrap(),
            choice => panic!("unexpected input character {}", choice),
        }
    }
}
