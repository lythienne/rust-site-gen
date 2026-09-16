/*#[derive(Debug)]
enum Serial<T> {
    In(T),
    Last(T),
    Empty
}

#[derive(Debug)]
enum Block {
    Text(Vec<Chunk>),
    Header(Vec<Chunk>, i8),
    Quote(Vec<Chunk>),
}

#[derive(Debug)]
struct Chunk {
    text: String,
    style: TextStyle
}

#[derive(Debug)]
enum TextStyle { Plain, Bold, Italic, SitelenPona }

pub fn compile(md: &str) -> String {
    let mut ir: Vec<Block> = Vec::new();
    let mut chars = md.chars();

    while let In(block) = parse_block(&mut chars) {
        ir.push(block);
    }    
    
    format!("{:?}", ir)
}

fn parse_block(chars: &mut impl Iterator<char>) -> Serial<Block> {
    let mut curr_block = Block::Text(Vec::new());

    while let In(chunk) = parse_chunk(&mut chars) {
        let Text(chunks) = curr_block;
        chunks.push(chunk);
    }
}

fn parse_chunk(chars: &mut impl Iterator<char>) -> Serial<Chunk> {
    let cnext = chars.next();
    if let None = cnext {
        return Empty;
    }

    let mut curr_chunk = {text: String::new(), style: TextStyle::Plain};

    while let Some(c) = cnext {
        match c {
            '*' => {
                return Some(curr_chunk);
            },
            other => {
                curr_chunk.text.push(other);
            },
        }
        cnext = chars.next();
    }
    return Some(curr_chunk);
}
*/
