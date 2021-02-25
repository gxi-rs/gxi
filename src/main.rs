/*
Each components is a tree
it initialises the tree when First
updates the attributes when not first on state change
when re-rendering renderer just has to go through the tree without touching the render function
 */


use crate::components::{Component, Text, View};

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
    fn get_sibling(&self) -> Option<&Box<dyn Component>> {
        self.sibling.as_ref()
    }

    fn get_sibling_mut(&mut self) -> Option<&mut Box<dyn Component>> {
        self.sibling.as_mut()
    }

    fn get_child(&self) -> Option<&Box<dyn Component>> {
        self.child.as_ref()
    }

    fn get_child_mut(&mut self) -> Option<&mut Box<dyn Component>> {
        self.child.as_mut()
    }

    fn render(&mut self, init: bool) {
        if init {
            self.child = Some(Box::from(View::default()));
        }
        let parent = self.child.as_mut().unwrap();
        //set attributes and children of child
        {
            let mut prev_sibling;
            // 1st is child
            {
                // init and set static values
                if init {
                    parent.get_sibling_mut().map(|_| Box::from(View { ..Default::default() }));
                }
                let node = parent.get_child_mut().unwrap();
                // set attributes which depend on variables
                // now node becomes parent for its children
                let parent = node;
                {
                    let prev_sibling;
                    // 1st is child
                    {
                        // init and set static values
                        if init {
                            parent.get_sibling_mut().map(|_| Box::from(Text {
                                label: String::from("Welcome"),
                                ..Default::default()
                            }));
                        }
                        let node = parent.get_child_mut().unwrap();
                        // set attributes which depend on variables
                        // set prev_sibling to current
                        prev_sibling = node;
                    }
                }
                // now parent i.e node becomes prev_sibling for the next sibling
                prev_sibling = parent;
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
        if self.counter == 0 {Text("Click button to update)} else {Text(self.counter)},
        Button("Press Me", on_click: |_| update(Msg::Inc))
    }
  }
}
*/