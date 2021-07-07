pub trait NativeWidget {
    fn append(&mut self, widget: &dyn NativeWidget);
    fn move_to_index(&mut self, widget: &dyn NativeWidget, index: usize);
}
