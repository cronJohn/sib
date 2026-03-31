#[derive(Default, Debug, Clone, Copy)]
pub struct Selection {
    index: usize,
}

impl Selection {
    pub fn new() -> Self {
        Self { index: 0 }
    }

    pub fn get(&self) -> usize {
        self.index
    }

    pub fn set(&mut self, idx: usize, len: usize) {
        self.index = idx;
        self.clamp(len);
    }

    pub fn up(&mut self) {
        self.index = self.index.saturating_sub(1);
    }

    pub fn down(&mut self, len: usize) {
        if self.index + 1 < len {
            self.index += 1;
        }
    }

    pub fn clamp(&mut self, len: usize) {
        if len == 0 {
            self.index = 0;
        } else if self.index >= len {
            self.index = len - 1;
        }
    }

    /// Move selection down, skipping non-selectable items
    pub fn move_down<T, F>(&mut self, items: &[T], is_selectable: F)
    where
        F: Fn(&T) -> bool,
    {
        let mut i = self.index + 1;
        while i < items.len() {
            if is_selectable(&items[i]) {
                self.index = i;
                return;
            }
            i += 1;
        }
        // if no selectable item below, keep current index
    }

    /// Move selection up, skipping non-selectable items
    pub fn move_up<T, F>(&mut self, items: &[T], is_selectable: F)
    where
        F: Fn(&T) -> bool,
    {
        let mut i = self.index.saturating_sub(1);
        loop {
            if is_selectable(&items[i]) {
                self.index = i;
                return;
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
        // if no selectable item above, keep current index
    }
}
