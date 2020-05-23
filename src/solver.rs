#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy)]
#[derive(Clone)]
pub enum Block {
    Unknown,
    Empty,
    Full
}


pub fn get_common(inp:usize, inp_line: &[Block], fill_to_end:bool) -> Vec<Block> {

    let len= inp_line.len();
    let mut all_lines: Vec<Vec<Block>> = vec![];
    let mut start = 0;
    let mut stop = len;

    for x2 in 0..len {
        if inp_line[x2] == Block::Full {
            stop = x2+1;
            println!("Stop: {:?}", stop);
        }

        if start == 0 && x2 + inp < len && inp_line[x2+inp] == Block::Full {
            start = x2;
            println!("Start: {:?}", start);
        }
    }

    for x in start..stop {

        if x + inp > len || x > stop {
            break;
        }

        println!("Verifying: {:?}", x);


        let mut line: Vec<Block> = vec![];
        let mut can_start = true;

        for x3 in 0..len {

            if x > 0 && x3 <= (x - 1) && inp_line[x3] == Block::Full {
                can_start = false;
                break;
            }
            else if x > 0 && x3 == (x - 1) {
                line.push(Block::Empty);
            }
            else if x3 >= x + inp && inp_line[x3] == Block::Full {
                can_start = false;
                break;
            }
            else if x3 == x + inp {
                line.push(Block::Empty);
            }
            else if fill_to_end {
                line.push(Block::Empty);
            }
            else if x3 < x {
                line.push(Block::Empty);
            }
            else {
                line.push(Block::Unknown);
            }
        }

        if can_start == false {
            continue;
        }


        let mut x2 = x;
        while x2 < x + inp {

            if inp_line[x2] == Block::Empty {
                break;
            }

            line[x2] = Block::Full;
            x2 += 1;
        }

        if x2 == x + inp {
            all_lines.push(line);
        }
    }

    if all_lines.len() == 0 {
        return vec![];
    }

    let mut common: Vec<Block> = vec![];

    for x in 0..len {
        let first = all_lines[0][x];
        let mut block : Block = first;
        for line in 0..all_lines.len() {
            let cur = all_lines[line][x];
            if cur != block {
                block = Block::Unknown;
                break;
            }
        }
        common.push(block);
    }

    println!("{:?}", all_lines);
    println!("");
    println!("Common: {:?}", common);

    return common;
}





#[test]
fn test_get_common_6_of_10() {

    let inp_line = [Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown];
    let ret = get_common(6, &inp_line, true);

    assert_eq!(ret, [Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown, Block::Full, Block::Full, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown]);
}

#[test]
fn test_get_common_2_set_first_empty_when_one_is_full() {

    let inp_line = [Block::Unknown, Block::Unknown, Block::Full, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown];
    let ret = get_common(2, &inp_line, true);

    assert_eq!(ret, [Block::Empty, Block::Unknown, Block::Full, Block::Unknown, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty]);
}

#[test]
fn test_get_common_2_set_first_empty_when_one_is_full_end() {

    let inp_line = [Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown, Block::Full, Block::Unknown, Block::Unknown];
    let ret = get_common(2, &inp_line, true);

    assert_eq!(ret, [Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Unknown, Block::Full, Block::Unknown, Block::Empty]);
}

#[test]
fn test_get_common_2_set_first_empty_but_dont_fill_to_end() {

    let inp_line = [Block::Unknown, Block::Unknown, Block::Full, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown];
    let ret = get_common(2, &inp_line, false);

    assert_eq!(ret, [Block::Empty, Block::Unknown, Block::Full, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown]);
}

#[test]
fn test_set_first() {

    let inp_line = [Block::Full, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown];
    let ret = get_common(2, &inp_line, true);

    assert_eq!(ret, [Block::Full, Block::Full, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty, Block::Empty]);
}

#[test]
fn empty_vector_is_returned_when_nothing_is_possible() {

    let inp_line = [Block::Unknown, Block::Unknown, Block::Unknown, Block::Empty, Block::Unknown, Block::Unknown, Block::Empty, Block::Unknown, Block::Unknown, Block::Unknown];
    let ret = get_common(4, &inp_line, true);

    assert_eq!(ret, []);

    assert_eq!(get_common(2, &[Block::Unknown, Block::Full, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown, Block::Full, Block::Unknown, Block::Unknown, Block::Unknown], true), []);
}
