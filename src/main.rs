/*
Each component is a tree
it initialises the tree when First
updates the attributes when not first on state change
when re-rendering renderer just has to go through the tree without touching the render function
 */

use std::collections::HashMap;

use button::Button;
use component::Component;
use text::Text;
use view::View;

use crate::component::Nodes;

mod component;
mod button;
mod view;
mod text;
mod run;

fn main() {
    run::run::<MyApp>();
}

struct MyApp {
    counter: i32,
    nodes: HashMap<i32, Box<dyn Component>>,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp {
            counter: 0,
            nodes: Default::default(),
        }
    }
}

impl Component for MyApp {
    fn get_nodes(&self) -> Option<&Nodes> {
        Some(&self.nodes)
    }

    fn get_mut_nodes(&mut self) -> Option<&mut Nodes> {
        Some(&mut self.nodes)
    }

    fn render(&self, tree: &mut HashMap<i32, Box<dyn Component>>, first: bool) {
        const KEY: i32 = 0;
        //initialize
        if first {
            tree.insert(KEY, Box::from(View::default()));
        }
        //set attributes or children
        match tree.get_mut(&KEY) {
            Some(tree) => {
                let tree = tree.get_mut_nodes().unwrap();
                {
                    const KEY: i32 = 0;
                    //root
                    if first {
                        tree.insert(KEY, Box::from(View::default()));
                    }
                    //Dont match if no attributes or off children
                }
                {
                    //if conditional render then first is replaced by condition
                    //branch out
                    const KEY: i32 = 1;

                    if self.counter == 0 {
                        tree.insert(KEY, Box::from(Text::default()));
                        // set attribute here
                    } else {
                        tree.insert(KEY, Box::from(Text {
                            label: "Hello".to_string()
                        }));
                        // set attribute here
                    }
                }
                {
                    const KEY: i32 = 3;

                    if first {
                        tree.insert(KEY, Box::from(Button {
                            label: "Press Me".to_string()
                        }));
                    }
                    //Dont match if no attributes or off children
                }
            }
            _ => {}
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
        View,
        if self.counter == 0 {Text(self.counter)} else {Text("Hello")},
        Button("Press Me", on_click: |_| update(Msg::Inc))
    }
  }
}
*/