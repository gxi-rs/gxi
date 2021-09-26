use crate::Observable;

pub fn use_state<T>(v: T) -> Observable<T> {
    Observable::new(v)
}
