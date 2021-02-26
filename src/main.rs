/*
Each components is a binary n tree
it initialises the tree when First
updates the attributes when not first on state change
when re-rendering renderer just has to go through the tree without touching the render function
*/
use std::any::Any;

use gtk::LabelExt;

use crate::components::{Button, Component, Node, Pure, Text, View};

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
    default_component!(false);

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
                        let node = parent.get_child_mut().get_or_insert_with(|| Box::from({
                            let node = Text::default();
                            node.widget.set_label("Welcome");
                            node
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
                        let node = parent.get_child_mut().get_or_insert_with(|| Box::from({
                            let node = Text::default();
                            node.widget.set_label("Un Set");
                            node
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
                            let text = node.as_any_mut().downcast_ref::<Text>().unwrap();
                            text.widget.set_label(&self.state.counter.to_string()[..]);
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
                let node = prev_sibling.get_sibling_mut().get_or_insert_with(|| {
                    let bt = Box::from(Button::default());
                    bt.set_label("PressMe");
                    bt.on_click(|| println!("Hello"));
                    bt
                });
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
        if self.counter == 0 {Text("Un Set")} else {Text(self.counter)},
        Button("Press Me", on_click: |_| update(Msg::Inc))
    }
  }
}
*/