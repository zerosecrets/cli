use dialoguer::console::Term;
use dialoguer::theme::Theme;
use dialoguer::Result;
use std::{fmt, io};
use termimad::crossterm::style::Stylize;

/// Helper struct to conveniently render a theme.
pub(crate) struct TermThemeRenderer<'a> {
    term: &'a Term,
    theme: &'a dyn Theme,
    height: usize,
    prompt_height: usize,
    prompts_reset_height: bool,
}

impl<'a> TermThemeRenderer<'a> {
    pub fn new(term: &'a Term, theme: &'a dyn Theme) -> TermThemeRenderer<'a> {
        TermThemeRenderer {
            term,
            theme,
            height: 0,
            prompt_height: 0,
            prompts_reset_height: true,
        }
    }

    fn write_formatted_line<
        F: FnOnce(&mut TermThemeRenderer, &mut dyn fmt::Write) -> fmt::Result,
    >(
        &mut self,
        f: F,
    ) -> Result {
        let mut buf = String::new();
        f(self, &mut buf).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        self.height += buf.chars().filter(|&x| x == '\n').count() + 1;
        Ok(self.term.write_line(&buf)?)
    }

    fn write_formatted_prompt<
        F: FnOnce(&mut TermThemeRenderer, &mut dyn fmt::Write) -> fmt::Result,
    >(
        &mut self,
        f: F,
    ) -> Result {
        self.write_formatted_line(f)?;
        if self.prompts_reset_height {
            self.prompt_height = self.height;
            self.height = 0;
        }
        Ok(())
    }

    pub fn sort_prompt(
        &mut self,
        prompt: &String,
        paging_info: Option<(usize, usize)>,
        description: &String,
    ) -> Result {
        self.write_formatted_prompt(|this, buf| {
            write!(buf, "  {}\n", description)?;

            if let Some(paging_info) = paging_info {
                write!(
                    buf,
                    "  [Page {}/{}]  {}\n",
                    paging_info.0,
                    paging_info.1,
                    "Use <Left>/<Right> to change the page and <Esc>/<q> to exit".dark_grey()
                )?;
            } else {
                write!(buf, "  {}\n", "Use <Esc>/<q> to exit".dark_grey())?;
            }

            this.theme.format_sort_prompt(buf, prompt)?;
            Ok(())
        })
    }

    pub fn sort_prompt_item(&mut self, text: &str, picked: bool, active: bool) -> Result {
        self.write_formatted_line(|this, buf| {
            this.theme
                .format_sort_prompt_item(buf, text, picked, active)
        })
    }

    pub fn clear(&mut self) -> Result {
        self.term
            .clear_last_lines(self.height + self.prompt_height)?;
        self.height = 0;
        self.prompt_height = 0;
        Ok(())
    }

    pub fn clear_preserve_prompt(&mut self, size_vec: &[usize]) -> Result {
        let mut new_height = self.height;
        let prefix_width = 2;
        //Check each item size, increment on finding an overflow
        for size in size_vec {
            if *size > self.term.size().1 as usize {
                new_height += ((*size as f64 + prefix_width as f64) / self.term.size().1 as f64)
                    .ceil() as usize
                    - 1;
            }
        }

        self.term.clear_last_lines(new_height)?;
        self.height = 0;
        Ok(())
    }
}
