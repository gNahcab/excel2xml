pub trait Extract {
    fn extract_name(&self) -> Self;
}
impl Extract for String {
    fn extract_name(&self) -> Self {
        let pos = match self.rfind("/") {
            None => {
                return self.to_string();
            }
            Some(pos) => { pos }
        };
        self[pos..].to_string()
    }
}