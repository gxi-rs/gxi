use gxi::{gxi, update, Renderable, StrongNodeType, WeakNodeType};

enum Msg {
    Modify(i32),
}

// TODO: Move gxi::Component macro from derive to functional macro, auto add node and state fields
#[derive(gxi::Component)]
pub struct Counter {
    node: gxi::ContainerNode,
    state: gxi::State<CounterState>,
}

#[derive(Default)]
struct CounterState {
    counter: i32,
}

impl Renderable for Counter {
    fn render(this: &StrongNodeType) {
        // TODO: check if state.is_dirty() 
        gxi! {
            div [
                {
                    let __this_borrow = this.as_ref().borrow();
                    let state = &__this_borrow.as_ref().downcast_ref::<Self>().unwrap().state;
                },
                h3 ( *inner_html = &state.counter.to_string()[..]),
                div [
                    button ( inner_html = "+" , on_click = update!(Msg::Modify(1)) ),
                    button ( inner_html = "-" , on_click = update!(Msg::Modify(-1)) ),
                ]
            ]
        }
    }
}

impl Counter {
    fn update(this: &WeakNodeType, msg: Msg) {
        if let Some(this) = this.upgrade() {
            {
                //TODO: create an update functional proc macro
                let mut __this_borrow = this.as_ref().borrow_mut();
                let state = &mut __this_borrow.as_mut().downcast_mut::<Self>().unwrap().state;

                match msg {
                    Msg::Modify(by) => {
                        state.counter += by;
                    }
                }
            }
            Self::render(&this)
        }
    }
}
