use crate::invisibilities::INVISIBILITIES;

pub struct InvisibleChecker {
    invisibilities: Vec<char>,
}

impl InvisibleChecker {
    pub fn builder() -> InvisibleCheckerBuilder {
        InvisibleCheckerBuilder {
            invisibilities: INVISIBILITIES.to_vec(),
        }
    }

    pub fn has_invisible_char(&self, name: &str) -> bool {
        for c in self.invisibilities.iter() {
            if name.contains(*c) {
                return true;
            }
        }

        false
    }

    pub fn invisible_list(&self) -> &Vec<char> {
        &self.invisibilities
    }

    /// Remove invisible characters from the left of the name.
    pub fn left_trim<'a>(&self, name: &'a str) -> &'a str {
        name.trim_start_matches(self.invisibilities.as_slice())
    }

    /// Remove invisible characters from the right of the name.
    pub fn right_trim<'a>(&self, name: &'a str) -> &'a str {
        name.trim_end_matches(self.invisibilities.as_slice())
    }

    /// Remove invisible characters from both sides of the name.
    pub fn trim<'a>(&self, name: &'a str) -> &'a str {
        name.trim_matches(self.invisibilities.as_slice())
    }
}

pub struct InvisibleCheckerBuilder {
    invisibilities: Vec<char>,
}

impl InvisibleCheckerBuilder {
    pub fn remove(&mut self, remove_char: char) -> &mut Self {
        self.invisibilities.retain(|c| *c != remove_char);
        self
    }

    pub fn removes(&mut self, remove_chars: &[char]) -> &mut Self {
        self.invisibilities.retain(|c| !remove_chars.contains(c));
        self
    }

    pub fn build(&self) -> InvisibleChecker {
        InvisibleChecker {
            invisibilities: self.invisibilities.clone(),
        }
    }
}