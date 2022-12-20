pub struct MsgGrabber {
    pub start_cursor: usize,
    start_code_width: usize,
}

impl MsgGrabber {
    pub fn new(start_code_width: usize) -> Self {
        MsgGrabber {
            start_cursor: 0,
            start_code_width: start_code_width
        }
    }

    pub fn find_start_code(&mut self, msg: &str) -> usize {
        for (index, token) in msg.chars().enumerate() {
            // look in the substring we have up to this point. remember slicing
            // doesn't include the end index
            let slice = &msg[self.start_cursor..index];
            match msg[self.start_cursor..index].rfind(token) {
                // we found a duplicate character
                Some(index) => {
                    // start over at the first character past the duplicate one
                    // we gotta add to the current start_cursor as the base
                    self.start_cursor = self.start_cursor + index + 1;
                },
                None => {
                    // we didn't find a duplicate character...is the current msg code
                    // long enough?
                    // gotta add one, because we include the character at index
                    if ((index - self.start_cursor) + 1) == self.start_code_width {
                        return index;
                    }
                }
            }
        }
        return 0;
    }
}