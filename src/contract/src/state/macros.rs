#[macro_export]
macro_rules! impl_deref {
    ($wrapper:ty, $target:ty) => {
        impl std::ops::Deref for $wrapper {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $wrapper {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}