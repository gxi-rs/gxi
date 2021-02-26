#[macro_export]
macro_rules! default_component {
    (false) => {
        fn get_sibling(&self) -> &Node { &self.sibling }

        fn get_sibling_mut(&mut self) -> &mut Node { &mut self.sibling }

        fn get_child(&self) -> &Node { &self.child }

        fn get_child_mut(&mut self) -> &mut Node { &mut self.child}

        fn as_any(&self) -> &dyn Any { self }

        fn as_any_mut(&mut self) -> &mut dyn Any { self }
    };
    (true)=>{
         default_component!(false);
         fn get_widget(&self) -> Option<&Widget> { Some(self.widget.as_ref()) }
    };
}
