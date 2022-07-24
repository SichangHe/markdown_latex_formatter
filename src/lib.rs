use std::error::Error;

pub fn format(content: &str) -> Result<String, Box<dyn Error>> {
    let mut list: Vec<_> = content.split("").collect();
    let mut state = State::Normal;
    let mut char = Char::Newline;
    let mut index = 0;
    while index < list.len() {
        // ! deal with math {
        if let State::Math(math) = &state {
            // todo: math stuff
            continue;
        } else if list[index] == "$" {
            if let Char::Backslash = char {
                state = State::Normal;
            } else if list[index + 1] == "$" {
                state = State::Math(Math::Block);
                (list, index) = ensure_newline(list, index);
                index += 2;
                if list[index] != "\n" {
                    list.insert(index, "\n");
                    index += 1;
                }
            } else {
                state = State::Math(Math::Inline);
            }
            index += 1;
            continue;
        } // ! }

        // ! deal with code exit {
        if let State::Code(code) = &state {
            if list[index] == "`" {
                match code {
                    Code::Inline => {
                        state = State::Normal;
                        index += 1;
                    }
                    Code::Block => {
                        if list[index + 1] == "`" && list[index + 2] == "`" {
                            state = State::Normal;
                            index += 3;
                            list.insert(index, "\n");
                            index += 1;
                            state = State::BlockEnd;
                        }
                    }
                }
            }
            continue;
        } // ! }

        match char {
            // ! followed by new line {
            Char::Newline => {
                if let State::BlockEnd = &state {
                    if list[index] != "\n" {
                        (list, index) = ensure_newline(list, index);
                    }
                }
                match list[index] {
                    "#" => {
                        state = State::Heading;
                        (list, index) = ensure_newline(list, index);
                        char = Char::Other;
                    }
                    "-" => match list[index + 1] {
                        " " => {
                            state = State::List(List::Unordered);
                            (list, index) = ensure_newline(list, index);
                        }
                        _ => state = State::Normal,
                    },
                    "1" => match list[index + 1] {
                        // catch: only work on 1
                        " " => {
                            state = State::List(List::Ordered);
                            (list, index) = ensure_newline(list, index);
                        }
                        _ => state = State::Normal,
                    },
                    "`" => {
                        if list[index + 1] == "`" && list[index + 2] == "`" {
                            state = State::Code(Code::Block);
                            (list, index) = ensure_newline(list, index);
                            index += 2;
                        } else {
                            state = State::Code(Code::Inline);
                        }
                    }
                    "\n" => {
                        while index + 1 < list.len() && list[index + 1] == "\n" {
                            list.remove(index);
                        }
                    }
                    _ => state = State::Normal,
                }
            } // ! }

            //…
            _ => state = State::Normal,
        }
        //…
        index += 1;
    }
    list = trim_beginning(list);
    Ok(String::from_iter(list))
}

fn ensure_newline(mut list: Vec<&str>, index: usize) -> (Vec<&str>, usize) {
    if list[index.saturating_sub(2)] == "\n" {
        (list, index)
    } else {
        list.insert(index, "\n");
        (list, index + 1)
    }
}

fn trim_beginning(mut list: Vec<&str>) -> Vec<&str> {
    while list[0] == "\n" {
        list.remove(0);
    }
    list
}

#[non_exhaustive]
enum Char {
    Other,
    Newline,
    Backslash,
}

enum Math {
    Inline,
    Block,
}

enum List {
    Ordered,
    Unordered,
}

enum Code {
    Inline,
    Block,
}

#[non_exhaustive]
enum State {
    Normal,
    Heading,
    List(List),
    Code(Code),
    Math(Math),
    Comment,
    BlockEnd,
}
