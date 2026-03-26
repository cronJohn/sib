use ratatui::widgets::ListState;

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

    pub fn up(&mut self, len: usize) {
        if len == 0 {
            self.index = 0;
        } else {
            self.index = self.index.saturating_sub(1);
        }
    }

    pub fn down(&mut self, len: usize) {
        if len == 0 {
            self.index = 0;
        } else if self.index + 1 < len {
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

    pub fn apply_to_list_state(&self, state: &mut ListState) {
        state.select(Some(self.index));
    }

    pub fn move_up<T, F>(&mut self, items: &[T], is_selectable: F)
    where
        F: Fn(&T) -> bool,
    {
        if items.is_empty() {
            self.index = 0;
            return;
        }

        let mut i = self.index.saturating_sub(1);

        loop {
            if is_selectable(&items[i]) {
                self.index = i;
                break;
            }

            if i == 0 {
                break;
            }

            i -= 1;
        }
    }

    pub fn move_down<T, F>(&mut self, items: &[T], is_selectable: F)
    where
        F: Fn(&T) -> bool,
    {
        if items.is_empty() {
            self.index = 0;
            return;
        }

        let mut i = self.index + 1;

        while i < items.len() {
            if is_selectable(&items[i]) {
                self.index = i;
                break;
            }
            i += 1;
        }
    }

    pub fn select_first<T, F>(&mut self, items: &[T], is_selectable: F)
    where
        F: Fn(&T) -> bool,
    {
        if let Some(i) = items.iter().position(is_selectable) {
            self.index = i;
        } else {
            self.index = 0;
        }
    }
}
