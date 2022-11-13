use crate::DynAccountRepository;

pub struct Context {
    pub account_repository: DynAccountRepository,
}

#[cfg(test)]
mod test {

    use std::sync::Arc;

    use crate::MockAccountRepository;

    impl super::Context {
        pub fn fake() -> Self {
            Self {
                account_repository: Arc::new(MockAccountRepository::default()),
            }
        }
    }
}
