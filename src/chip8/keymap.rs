#[derive(Default)]
pub(crate) struct Keymap {
    inner: [bool; 16],
}

impl Keymap {
    pub fn set(&mut self, index: usize) {
        self.inner[index] = true;
    }
    pub fn is_down(&self, index: usize) -> bool {
        self.inner[index]
    }
    pub fn or(&self, other: &Keymap) -> Keymap {
        let inner: [bool; 16] = self
            .inner
            .iter()
            .zip(other.inner)
            .map(|(key, other_key)| key | other_key)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Keymap { inner }
    }
    pub fn down_to_up(&self, other: &Keymap) -> Option<usize> {
        self.inner
            .iter()
            .zip(other.inner)
            .enumerate()
            .find(|(_, (key, other_key))| key == &&true && other_key == &false)
            .map(|(index, _)| index)
    }
}
