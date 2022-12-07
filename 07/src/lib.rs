pub struct CumulativeDirSizer {
    stack: Vec<usize>,
    sizes: Vec<usize>,
}

impl CumulativeDirSizer {
    pub fn new() -> Self {
        Self {
            stack: vec![0],
            sizes: vec![],
        }
    }

    pub fn process(&mut self, line: &str) {
        if line == "$ cd /" || line == "$ ls" || line.starts_with("dir ") {
            // NOOP
        } else if line.starts_with("$ cd ..") {
            self.go_up();
        } else if line.starts_with("$ cd ") {
            self.go_down();
        } else {
            let size = line.split_once(" ").unwrap().0.parse::<usize>().unwrap();
            self.add_size(size);
        }
    }

    fn go_up(&mut self) {
        let size = self.stack.pop().unwrap();
        self.sizes.push(size);
        if let Some(top) = self.stack.last_mut() {
            *top += size;
        }
    }

    fn go_down(&mut self) {
        self.stack.push(0);
    }

    fn add_size(&mut self, size: usize) {
        let top = self.stack.last_mut().unwrap();
        *top += size;
    }

    pub fn get_sizes(&mut self) -> &[usize] {
        while !self.stack.is_empty() {
            self.go_up();
        }
        return &self.sizes;
    }
}
