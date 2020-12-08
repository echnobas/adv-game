mod choice;
mod tree;
use choice::{
    Choice,
    ChoiceMachine,
};
use tree::StoryNode;

use prompt_rs::prompt::Prompt;

fn main() {
    let tree = StoryNode::new(
        "You wake up in a forest covered in snow",
        vec![Box::new(StoryNode::new(
            "You hear a howling in the woods",
            vec![Box::new(StoryNode::new(
                "A group of 3 wolves confront you",
                vec![
                    Box::new(StoryNode::new(
                        "One of the wolves flees, the other two begin to attack you",
                        vec![Box::new(StoryNode::new(
                            "You die",
                            vec![],
                            ChoiceMachine::new(&[]),
                        ))],
                        ChoiceMachine::new(&[Choice::A("Die")]),
                    )),
                    Box::new(StoryNode::new(
                        "The wolves growl at you and flee",
                        vec![Box::new(StoryNode::new(
                            "The edge of the forest is on the horizon, you can escape",
                            vec![Box::new(StoryNode::new(
                                "You escape the forest",
                                vec![],
                                ChoiceMachine::new(&[]),
                            ))],
                            ChoiceMachine::new(&[Choice::A("Escape")]),
                        ))],
                        ChoiceMachine::new(&[Choice::A("Continue east"), Choice::B("Head back")]),
                    )),
                ],
                ChoiceMachine::new(&[Choice::A("Shoot"), Choice::B("Don't shoot")]),
            ))],
            ChoiceMachine::new(&[
                Choice::A("Draw your revolver"),
                Choice::B("Light a torch"),
                Choice::C("Draw your knife"),
                Choice::D("Sprint away"),
            ]),
        ))],
        ChoiceMachine::new(&[
            Choice::A("Head north"),
            Choice::B("Head east"),
            Choice::C("Head south"),
            Choice::D("Head west"),
        ]),
    );
    evaluate_story(tree);
}

fn evaluate_story(tree: StoryNode) {
    println!("{}", tree.prompt);
    let result = tree.choices.exec();
    match result {
        Choice::A(_) => evaluate_story(*tree.a.expect("End of story")),
        Choice::B(_) => evaluate_story(*tree.b.expect("End of story")),
        Choice::C(_) => evaluate_story(*tree.c.expect("End of story")),
        Choice::D(_) => evaluate_story(*tree.c.expect("End of story")),
    }
}
