use std::any::{Any, TypeId};
use std::collections::HashMap;

use std::rc::Rc;

#[derive(Default)]
pub struct DiContainer {
    map: HashMap<TypeId, Rc<dyn Any>>,
}

impl DiContainer {
    pub fn insert<T>(&mut self, service: T)
    where
        T: Any,
    {
        self.map.insert(TypeId::of::<T>(), Rc::new(service));
    }

    pub fn get<T>(&self) -> Option<Rc<T>>
    where
        T: Any,
    {
        self.map
            .get(&TypeId::of::<T>())
            .map(Rc::clone)?
            .downcast()
            .ok()
    }
}

pub trait ServiceFn<Args> {
    type Output;

    fn exec(&self, di_container: &DiContainer) -> Self::Output;
}

macro_rules! impl_service_fn {
    ($($T:ident),+) => {
        impl<F, $($T: Any),*, R> ServiceFn<($($T),*, R)> for F
        where
            F: Fn($(&$T),*) -> R,
        {
            type Output = R;

            fn exec(&self, di_container: &DiContainer) -> Self::Output {
                self($(&di_container.get::<$T>().unwrap()),*)
            }
        }
    };
}

impl_service_fn!(A1);
impl_service_fn!(A1, A2);
impl_service_fn!(A1, A2, A3);
impl_service_fn!(A1, A2, A3, A4);
impl_service_fn!(A1, A2, A3, A4, A5);

#[cfg(test)]
mod tests {
    use super::*;

    struct Check(u8);

    #[test]
    fn check_di_insertion() {
        let mut di_container = DiContainer::default();
        di_container.insert(Check(1));
        assert!(di_container.get::<Check>().is_some());
    }

    #[test]
    fn check_fn_call() {
        let mut di_container = DiContainer::default();
        di_container.insert(Check(1));
        let val = check.exec(&di_container);
        assert_eq!(val, 2);
    }

    #[test]
    fn check2_fn_call() {
        let mut di_container = DiContainer::default();
        di_container.insert(Check(2));
        let val = check2.exec(&di_container);
        assert_eq!(val, 4);
    }

    fn check(val: &Check) -> u8 {
        val.0 + 1
    }

    fn check2(val: &Check, val2: &Check) -> u8 {
        val.0 + val2.0
    }
}
