use crate::AccountRepository;

pub trait Context {
    type AccountRepository: AccountRepository;

    fn account_repository(&self) -> &Self::AccountRepository;
}

#[cfg(test)]
mod test {

    use crate::repository::AccountRepositoryContainer;

    use super::Context;

    struct FakeContext {
        account_repository: AccountRepositoryContainer,
    }

    impl Context for FakeContext {
        type AccountRepository = AccountRepositoryContainer;

        fn account_repository(&self) -> &Self::AccountRepository {
            &self.account_repository
        }
    }
}
