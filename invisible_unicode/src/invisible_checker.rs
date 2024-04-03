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