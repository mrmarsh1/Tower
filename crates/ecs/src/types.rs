pub trait Component {
    fn get_id() -> usize;
    fn entity(&self) -> usize;
    fn one_frame(&self) -> bool;
}
