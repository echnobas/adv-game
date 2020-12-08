use crate::{
    Choice,
    ChoiceMachine,
};

pub struct StoryNode<'choice_machine> {
    pub prompt: String,
    pub opt_count: u8,
    pub a: Option<Box<StoryNode<'choice_machine>>>,
    pub b: Option<Box<StoryNode<'choice_machine>>>,
    pub c: Option<Box<StoryNode<'choice_machine>>>,
    pub d: Option<Box<StoryNode<'choice_machine>>>,
    pub choices: ChoiceMachine<'choice_machine>,
}

impl<'choice_machine> StoryNode<'choice_machine> {
    pub fn new(
        prompt: &str,
        children: Vec<Box<StoryNode<'choice_machine>>>,
        choices: ChoiceMachine<'choice_machine>,
    ) -> Self {
        let mut children = children.into_iter();
        let opt_count = children.len() as u8;
        let a = children.next();
        let b = children.next();
        let c = children.next();
        let d = children.next();
        Self {
            prompt: prompt.to_owned(),
            opt_count,
            a,
            b,
            c,
            d,
            choices,
        }
    }
}
