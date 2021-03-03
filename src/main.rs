/*
Each components is a binary n tree
it initialises the tree when First
updates the attributes when not first on state change
when re-rendering renderer just has to go through the tree without touching the render function
*/
use std::any::Any;
use std::borrow::{Borrow, BorrowMut};
use std::cell::{RefCell, RefMut};
use std::ops::Deref;
use std::process::exit;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use gtk::{LabelExt, WidgetExt};
use gtk::prelude::WidgetExtManual;

use crate::components::{Button, Component, Node, Pure, Text, View};
use crate::Msg::Inc;

mod components;
mod run;

fn main() {
    run::run();
}

#[derive(Default)]
pub struct MyApp {
    state: Arc<Mutex<MyAppState>>,
    child: Option<Box<dyn Component>>,
    sibling: Option<Box<dyn Component>>,
}

#[derive(Default)]
struct MyAppState {
    counter: i32,
}

enum Msg {
    Inc,
    Dec,
}

impl MyApp {
    fn update(&self) {}
}

impl Component for MyApp {
    default_component!(false);

    #[allow(unused_assignments)]
    #[allow(unused_variables)]
    fn render(&mut self) {
        println!("render");
        let state = self.state.clone();
        let state = state.lock().unwrap();
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
                            node
                        }));
                        let text  = node.as_any_mut().downcast_ref::<Text>().unwrap();
                        text.widget.set_label(&format!("Count {}", state.counter)[..]);
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
                if state.counter == 0 {
                    if pure_block.type_extra != 0 {
                        pure_block.type_extra = 0;
                        pure_block.child.replace(Box::from({
                            let node = Text::default();
                            node.widget.set_label("Un Set");
                            node
                        }));
                    }
                    let parent = prev_sibling.get_sibling_mut().as_mut().unwrap();
                    let prev_sibling;
                    {
                        let node = parent.get_child_mut().as_mut().unwrap();
                        // set attributes which depend on variables
                        // now node becomes prev_sibling
                        prev_sibling = node;
                    }
                    //render
                    prev_sibling.render();
                } else {
                    if pure_block.type_extra != 1 {
                        pure_block.type_extra = 1;
                        pure_block.child.replace( Box::from(Text { ..Default::default() }));
                    }
                    let parent = prev_sibling.get_sibling_mut().as_mut().unwrap();
                    let prev_sibling;
                    {
                        let mut node = parent.get_child_mut().as_mut().unwrap();
                        // set attributes which depend on variables
                        {
                            let text = node.as_any_mut().downcast_ref::<Text>().unwrap();
                            println!("{}", &state.counter.to_string()[..]);
                            text.widget.set_label(&state.counter.to_string()[..]);
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
                let state = self.state.clone();

                let node = prev_sibling.get_sibling_mut().get_or_insert_with(move || {
                    let bt = Box::from(Button::default());
                    bt.set_label("PressMe");
                    bt.on_click(Box::from(move || {
                        println!("call");
                        {
                            let mut state = state.lock().unwrap();
                            state.counter += 1;
                        }
                        crate::run::APP.lock().unwrap().render();
                    }));
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