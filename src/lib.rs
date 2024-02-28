use std::{cell::UnsafeCell, mem::MaybeUninit};

pub enum SimpleCellError {
    AlreadyInitialized,
    ValueUnitialized,
}
pub struct SimpleCell<T> {
    value: UnsafeCell<MaybeUninit<T>>,
    initialized: UnsafeCell<bool>,
}

impl<T> Default for SimpleCell<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> SimpleCell<T> {
    pub fn new() -> Self {
        Self {
            value: UnsafeCell::new(MaybeUninit::uninit()),
            initialized: UnsafeCell::new(false),
        }
    }

    pub fn get(&self) -> Option<&T> {
        let get_ref = unsafe { self.value.get().as_ref()? };
        Some(unsafe { get_ref.assume_init_ref() })
    }

    pub fn set(&self, value_in: T) -> Result<(), SimpleCellError> {
        let mut_init = unsafe {
            self.initialized
                .get()
                .as_mut()
                .ok_or(SimpleCellError::ValueUnitialized)?
        };
        if *mut_init {
            return Err(SimpleCellError::AlreadyInitialized);
        }

        let mut_value = unsafe {
            self.value
                .get()
                .as_mut()
                .ok_or(SimpleCellError::ValueUnitialized)?
        };
        mut_value.write(value_in);

        *mut_init = true;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct TestA {
        data: [u8; 4],
    }
    #[test]
    fn test_simple_cell() {
        let simple_cell = SimpleCell::new();
        let data_test = [1, 1, 1, 1];
        assert!(simple_cell
            .set(TestA {
                data: data_test.clone()
            })
            .is_ok());

        assert_eq!(simple_cell.get(), Some(&TestA { data: data_test }));

        assert!(simple_cell.set(TestA { data: [1, 2, 1, 1] }).is_err());
    }
}
