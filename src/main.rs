mod solver;
use solver::Block;
use solver::get_common;

fn main() {



    let inp_line = [solver::Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown, Block::Unknown];
    let ret = get_common(6, &inp_line, true);

    println!("Hello, world! {:?}", ret);


}
