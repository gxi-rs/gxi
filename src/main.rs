/*
Each components is a binary n tree
it initialises the tree when First
updates the attributes when not first on state change
when re-rendering renderer just has to go through the tree without touching the render function
*/
use std::any::Any;

use crate::components::{Button, Component, Node, Pure, Text, View, Widget};

mod components;
mod run;

fn main() {
    run::run::<MyApp>();
}

#[derive(Default)]
struct MyApp {
    state: MyAppState,
    child: Option<Box<dyn Component>>,
    sibling: Option<Box<dyn Component>>,
}

#[derive(Default)]
struct MyAppState {
    counter: i32
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
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    #[allow(unused_assignments)]
    #[allow(unused_variables)]
    fn render(&mut self) {
        let parent = self.child.get_or_insert_with(|| Box::from(View::default()));
        //set attributes and children of child
        {
            let mut prev_sibling;
            // 1st is child
            {
                let node = parent.get_child_mut().get_or_insert_with(|| Box::from(View {
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
                        let node = parent.get_child_mut().get_or_insert_with(|| Box::from(Text {
                            label: String::from("Welcome"),
                            ..Default::default()
                        }));
                        // set attributes which depend on variables
                        // set prev_sibling to current
                        prev_sibling = node;
                    }
                    //render
                    prev_sibling.render();
                }
                //now parent i.e node becomes prev_sibling for the next sibling
                prev_sibling = parent;
            }
            //render
            prev_sibling.render();

            // 1st sibling
            {
                // when if statement create an empty pure element to pre occupy space
                let mut pure_block = prev_sibling.get_sibling_mut().get_or_insert_with(|| Box::from(Pure::default())).as_any_mut().downcast_mut::<Pure>().unwrap();
                // user defined if statement
                if self.state.counter == 0 {
                    if pure_block.type_extra != 0 {
                        pure_block.type_extra = 0;
                        pure_block.child = None;
                    }
                    let parent = prev_sibling.get_sibling_mut().as_mut().unwrap();
                    let prev_sibling;
                    {
                        let node = parent.get_child_mut().get_or_insert_with(|| Box::from(Text {
                            label: String::from("UnSet"),
                            ..Default::default()
                        }));
                        // set attributes which depend on variables
                        // now node becomes prev_sibling
                        prev_sibling = node;
                    }
                    //render
                    prev_sibling.render();
                } else {
                    if pure_block.type_extra != 1 {
                        pure_block.type_extra = 1;
                        pure_block.child = None;
                    }
                    let parent = prev_sibling.get_sibling_mut().as_mut().unwrap();
                    let prev_sibling;
                    {
                        let node = parent.get_child_mut().get_or_insert_with(|| Box::from(Text { ..Default::default() }));
                        // set attributes which depend on variables
                        {
                            let mut text = node.as_any_mut().downcast_mut::<Text>().unwrap();
                            text.label = self.state.counter.to_string();
                        }
                        // now node becomes prev_sibling
                        prev_sibling = node;
                    }
                    //render
                    prev_sibling.render();
                }
                prev_sibling = prev_sibling.get_sibling_mut().as_mut().unwrap();
            }
            //render
            prev_sibling.render();

            // 2nd sibling
            {
                let node = prev_sibling.get_sibling_mut().get_or_insert_with(|| Box::from(Button {
                    label: String::from("PressMe"),
                    ..Default::default()
                }));
                // set attributes which depend on variables
                // now node becomes prev_sibling
                prev_sibling = node;
            }
            //render
            prev_sibling.render();
        }
        parent.render();
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