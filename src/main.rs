/*
Each components is a binary n tree
it initialises the tree when First
updates the attributes when not first on state change
when re-rendering renderer just has to go through the tree without touching the render function
*/

use crate::components::{Button, Component, Node, Text, View};

mod components;
mod run;

fn main() {
    run::run::<MyApp>();
}

struct MyApp {
    counter: i32,
    child: Option<Box<dyn Component>>,
    sibling: Option<Box<dyn Component>>,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp {
            counter: 0,
            child: None,
            sibling: None,
        }
    }
}

impl Component for MyApp {
    fn get_sibling(&self) -> &Node { &self.sibling }

    fn get_sibling_mut(&mut self) -> &mut Node {
        &mut self.sibling
    }

    fn get_child(&self) -> &Node { &self.child }

    fn get_child_mut(&mut self) -> &mut Node {
        &mut self.child
    }

    fn render(&mut self) {
        let parent = self.child.get_or_insert(Box::from(View::default()));
        //set attributes and children of child
        {
            let mut prev_sibling;
            // 1st is child
            {
                let node = parent.get_child_mut().get_or_insert(Box::from(View {
                    ..Default::default()
                }));
                // set attributes which depend on variables
                // now node becomes parent for its children
                let parent = node;
                {
                    let prev_sibling;
                    // 1st is child
                    {
                        // init and set static values
                        let node = parent.get_child_mut().get_or_insert(Box::from(Text {
                            label: String::from("Welcome"),
                            ..Default::default()
                        }));
                        // set attributes which depend on variables
                        // set prev_sibling to current
                        prev_sibling = node;
                    }
                }
                // now parent i.e node becomes prev_sibling for the next sibling
                prev_sibling = parent;
            }

            // 1st sibling
            {
                let node = prev_sibling.get_sibling_mut().get_or_insert(Box::from(Text {
                    label: String::from("UnSet"),
                    ..Default::default()
                }));
                // set attributes which depend on variables
                // now node becomes prev_sibling
                prev_sibling = node;
            }
            // 2nd sibling
            {
                let node = prev_sibling.get_sibling_mut().get_or_insert(Box::from(Button {
                    label: String::from("PressMe"),
                    ..Default::default()
                }));
                // set attributes which depend on variables
                // now node becomes prev_sibling
                prev_sibling = node;
            }
        }
    }
}
/*
c! {
  MyApp
  Msg {
      Inc,
      Dec
  }
  state {
      counter:i32 = 0
  }
  update {
    match self.msg {
        Msg::Inc => {
            self.state.counter+=1;
        }
    }
  }
  render {
    View {
        View {
            Text("Welcome")
        },
        if self.counter == 0 {Text("UnSet")} else {Text(self.counter)},
        Button("Press Me", on_click: |_| update(Msg::Inc))
    }
  }
}
*/