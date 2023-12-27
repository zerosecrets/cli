use crate::common::list::{paging::Paging, render::TermThemeRenderer};
use dialoguer::{
    console::{Key, Term},
    theme::{SimpleTheme, Theme},
    Result,
};
use std::io;

#[derive(Clone)]
pub struct List<'a> {
    description: Option<String>,
    items: Vec<String>,
    prompt: Option<String>,
    clear: bool,
    max_length: Option<usize>,
    theme: &'a dyn Theme,
}

impl Default for List<'static> {
    fn default() -> Self {
        Self::new()
    }
}

impl List<'static> {
    pub fn new() -> Self {
        Self::with_theme(&SimpleTheme)
    }
}

impl List<'_> {
    pub fn clear(mut self, val: bool) -> Self {
        self.clear = val;
        self
    }

    pub fn max_length(mut self, val: usize) -> Self {
        self.max_length = Some(val + 2);
        self
    }

    pub fn items<T: ToString>(mut self, items: &[T]) -> Self {
        for item in items {
            self.items.push(item.to_string());
        }
        self
    }

    pub fn with_prompt<S: Into<String>>(mut self, prompt: S) -> Self {
        self.prompt = Some(prompt.into());
        self
    }

    pub fn with_description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn interact_opt(self) -> Result<Option<Vec<usize>>> {
        self.interact_on_opt(&Term::stderr())
    }

    pub fn interact_on_opt(self, term: &Term) -> Result<Option<Vec<usize>>> {
        self._interact_on(term, true)
    }

    fn _interact_on(self, term: &Term, allow_quit: bool) -> Result<Option<Vec<usize>>> {
        if !term.is_term() {
            return Err(io::Error::new(io::ErrorKind::NotConnected, "not a terminal").into());
        }

        if self.items.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Empty list of items given to `Sort`",
            ))?;
        }

        let mut paging = Paging::new(term, self.items.len(), self.max_length);
        let mut render = TermThemeRenderer::new(term, self.theme);
        let mut sel = 0;
        let mut size_vec = Vec::new();

        for items in self.items.iter().as_slice() {
            let size = &items.len();
            size_vec.push(*size);
        }

        let mut order: Vec<_> = (0..self.items.len()).collect();
        let checked: bool = false;
        term.hide_cursor()?;

        loop {
            if let Some(ref prompt) = self.prompt {
                if let Some(ref description) = self.description {
                    paging.render_prompt(|paging_info| {
                        render.sort_prompt(prompt, paging_info, description)
                    })?;
                }
            }

            for (idx, item) in order
                .iter()
                .enumerate()
                .skip(paging.current_page * paging.capacity)
                .take(paging.capacity)
            {
                render.sort_prompt_item(&self.items[*item], checked, sel == idx)?;
            }

            term.flush()?;

            match term.read_key()? {
                Key::ArrowLeft => {
                    if paging.active {
                        let old_sel = sel;
                        let old_page = paging.current_page;
                        sel = paging.previous_page();

                        if checked {
                            let indexes: Vec<_> = if old_page == 0 {
                                let indexes1: Vec<_> = (0..=old_sel).rev().collect();
                                let indexes2: Vec<_> = (sel..self.items.len()).rev().collect();
                                [indexes1, indexes2].concat()
                            } else {
                                (sel..=old_sel).rev().collect()
                            };

                            for index in 0..(indexes.len() - 1) {
                                order.swap(indexes[index], indexes[index + 1]);
                            }
                        }
                    }
                }
                Key::ArrowRight => {
                    if paging.active {
                        let old_sel = sel;
                        let old_page = paging.current_page;
                        sel = paging.next_page();

                        if checked {
                            let indexes: Vec<_> = if old_page == paging.pages - 1 {
                                let indexes1: Vec<_> = (old_sel..self.items.len()).collect();
                                let indexes2: Vec<_> = vec![0];
                                [indexes1, indexes2].concat()
                            } else {
                                (old_sel..=sel).collect()
                            };

                            for index in 0..(indexes.len() - 1) {
                                order.swap(indexes[index], indexes[index + 1]);
                            }
                        }
                    }
                }
                Key::Escape | Key::Char('q') => {
                    if allow_quit {
                        if self.clear {
                            render.clear()?;
                        } else {
                            term.clear_last_lines(paging.capacity)?;
                        }

                        term.show_cursor()?;
                        term.flush()?;
                        return Ok(None);
                    }
                }
                _ => {}
            }

            paging.update(sel)?;

            if paging.active {
                render.clear()?;
            } else {
                render.clear_preserve_prompt(&size_vec)?;
            }
        }
    }
}

impl<'a> List<'a> {
    pub fn with_theme(theme: &'a dyn Theme) -> Self {
        Self {
            items: vec![],
            clear: true,
            prompt: None,
            max_length: None,
            description: None,
            theme,
        }
    }
}
