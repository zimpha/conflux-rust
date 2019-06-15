pub enum VerificationMsg {
    Hello,
    World,
}

pub struct BlockVerifier {}

impl BlockVerifier {
    pub fn new() -> Self { Self {} }
}
