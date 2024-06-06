use super::{Parser, Result, Tokens};

#[derive(Debug, Clone)]
pub enum Token {
    Wildcard,
    Nat(u32),
    Int(i32),
    Float(f32),
    String(String),
    Identifier(String),
    Operator(Operator),
    Symbol(Symbol),
    Keyword(Keyword),
}

#[derive(Debug, Clone)]
pub struct LocatedToken {
    token: Token,
    position: usize,
}

#[derive(Debug, Clone)]
pub enum Keyword {
    Module,
    Import,
    Let,
    If,
    Then,
    Else,
    When,
    Is,
}

#[derive(Debug, Clone)]
pub enum Symbol {
    OpenParen,
    CloseParen,
    OpenSquareParen,
    CloseSquareParen,
    OpenSquigglyParen,
    CloseSquigglyParen,
    EmptyList,
    Eq,
    Alt,
    Arrow,
    Semicolon,
    Period,
    Comma,
    Colon,
    DoubleColon,
}

fn lexeme<'a, F: 'a, O>(inner: F) -> impl FnMut(Tokens) -> Result<O>
where
    F: FnMut(Tokens) -> Result<O>,
{
    delimited(multispace0, inner, multispace0)
}

fn operator(i: Span) -> Result<LocatedToken> {
    let (i, pos) = position(i)?;
    let (i, token) = lexeme(alt((
        tag("-").map(|_| Operator::Minus),
        tag("<<").map(|_| Operator::Compose),
        tag(">>").map(|_| Operator::ComposeRev),
        tag("^").map(|_| Operator::Power),
        tag("*").map(|_| Operator::Times),
        tag("/").map(|_| Operator::Divide),
        tag("%").map(|_| Operator::Mod),
        tag("+").map(|_| Operator::Plus),
        tag("-").map(|_| Operator::Minus),
        tag("++").map(|_| Operator::Concat),
        tag("==").map(|_| Operator::Eq),
        tag("!=").map(|_| Operator::Neq),
        tag("<").map(|_| Operator::LT),
        tag("<=").map(|_| Operator::LTE),
        tag(">=").map(|_| Operator::GTE),
        tag(">=").map(|_| Operator::GT),
        tag("&&").map(|_| Operator::And),
        tag("||").map(|_| Operator::Or),
        tag("<|").map(|_| Operator::Pipe),
        tag("|>").map(|_| Operator::PipeRev),
    )))
    .map(Token::Operator)
    .parse(i)?;

    success(LocatedToken {
        position: pos,
        token,
    })(i)
}

fn keyword<'a>(i: Span) -> Result<LocatedToken> {
    let (i, pos) = position(i)?;
    let (i, token) = lexeme(alt((
        tag("module").map(|_| Keyword::Module),
        tag("import").map(|_| Keyword::Import),
        tag("let").map(|_| Keyword::Let),
        tag("if").map(|_| Keyword::If),
        tag("then").map(|_| Keyword::Then),
        tag("else").map(|_| Keyword::Else),
        tag("when").map(|_| Keyword::When),
        tag("is").map(|_| Keyword::Is),
    )))
    .map(Token::Keyword)
    .parse(i)?;

    success(LocatedToken {
        position: pos,
        token,
    })(i)
}

fn symbol<'a>(i: Span) -> Result<LocatedToken> {
    let (i, pos) = position(i)?;
    let (i, token) = lexeme(alt((
        tag("->").map(|_| Symbol::Arrow),
        tag("[]").map(|_| Symbol::EmptyList),
        tag("::").map(|_| Symbol::DoubleColon),
        tag("(").map(|_| Symbol::OpenParen),
        tag(")").map(|_| Symbol::CloseParen),
        tag("[").map(|_| Symbol::OpenSquareParen),
        tag("]").map(|_| Symbol::CloseSquareParen),
        tag("{").map(|_| Symbol::OpenSquigglyParen),
        tag("}").map(|_| Symbol::CloseSquigglyParen),
        tag("=").map(|_| Symbol::Eq),
        tag("|").map(|_| Symbol::Alt),
        tag(";").map(|_| Symbol::Semicolon),
        tag(".").map(|_| Symbol::Period),
        tag(",").map(|_| Symbol::Comma),
        tag(":").map(|_| Symbol::Colon),
    )))
    .map(Token::Symbol)
    .parse(i)?;

    success(LocatedToken {
        position: pos,
        token,
    })(i)
}

fn value_identifier(i: Span) -> Result<LocatedToken> {
    let (i, pos) = position(i)?;
    let (i, first) = satisfy(|c| (c.is_alphabetic() && c.is_lowercase()) || c == '_')(i)?;
    let (i, mut rest) = many0(satisfy(|c| {
        (c.is_alphabetic() && c.is_lowercase()) || c == '_' || c.is_numeric()
    }))(i)?;
    let (i, mut end) = many0(satisfy(|c| c == '?'))(i)?;
    let mut ident: Vec<char> = vec![first];
    ident.append(&mut rest);
    ident.append(&mut end);

    success(LocatedToken {
        position: pos,
        token: Token::Identifier(ident.into_iter().collect::<String>()),
    })(i)
}

fn type_identifier(i: Span) -> Result<LocatedToken> {
    let (i, pos) = position(i)?;
    let (i, first) = satisfy(|c| (c.is_alphabetic() && c.is_uppercase()))(i)?;
    let (i, mut rest) = many0(satisfy(|c| c.is_alphabetic() || c.is_numeric()))(i)?;
    let (i, mut end) = many0(satisfy(|c| c == '?'))(i)?;
    let mut ident: Vec<char> = vec![first];
    ident.append(&mut rest);
    ident.append(&mut end);

    success(LocatedToken {
        position: pos,
        token: Token::Identifier(ident.into_iter().collect::<String>()),
    })(i)
}

fn nat(i: Span) -> Result<LocatedToken> {
    let (i, pos) = position(i)?;
    let (i, token) = digit1
        .map(|digits: Span| Token::Nat(digits.parse::<u32>().unwrap()))
        .parse(i)?;

    success(LocatedToken {
        position: pos,
        token,
    })(i)
}

fn int(i: Span) -> Result<LocatedToken> {
    let (i, pos) = position(i)?;
    let (i, token) = terminated(
        tuple((
            alt((tag("-").map(|_| -1), tag("+").map(|_| 1), success(1))),
            digit1.map(|string: Span| string.parse::<i32>().unwrap()),
        )),
        not(tag(".")),
    )
    .map(|(sign, num)| sign * num)
    .map(Token::Int)
    .parse(i)?;

    success(LocatedToken {
        position: pos,
        token,
    })(i)
}

fn string_literal(i: Span) -> Result<LocatedToken> {
    let (i, pos) = position(i)?;
    let (i, token) = preceded(
        tag("\""),
        many_till(satisfy(|_| true), tag("\""))
            .map(|(chars, _)| Token::String(chars.into_iter().collect::<String>())),
    )
    .parse(i)?;

    success(LocatedToken {
        position: pos,
        token,
    })(i)
}

fn token(i: Span) -> Result<LocatedToken> {
    alt((
        keyword,
        symbol,
        operator,
        type_identifier,
        value_identifier,
        nat,
        int,
        string_literal,
    ))(i)
}

pub fn tokenize<'a>(i: String) -> Result<'a, Tokens<'a>> {
    let i = Span::new(&i);
    let (_, tokens) = terminated(many0(token), eof)(i)?;
    success(Tokens::new(tokens))(i)
}

pub fn unit(i: Tokens) -> Result<()> {
    todo!()
}

pub fn nat(i: Tokens) -> Result<u32> {
    todo!()
}
pub fn int(i: Tokens) -> Result<i32> {
    todo!()
}
pub fn float(i: Tokens) -> Result<f32> {
    todo!()
}
pub fn string(i: Tokens) -> Result<&str> {
    todo!()
}

pub fn value_identifier(i: Tokens) -> Result<&str> {
    todo!()
}
pub fn type_identifier(i: Tokens) -> Result<&str> {
    todo!()
}

pub fn kw(k: Keyword) -> impl FnMut(Tokens) -> Result<()> {
    move |i| todo!()
}

pub fn alt(i: Tokens) -> Result<()> {
    todo!()
}
pub fn backslash(i: Tokens) -> Result<()> {
    todo!()
}
pub fn arrow(i: Tokens) -> Result<()> {
    todo!()
}
pub fn back_arrow(i: Tokens) -> Result<()> {
    todo!()
}
pub fn wildcard(i: Tokens) -> Result<()> {
    todo!()
}
pub fn period(i: Tokens) -> Result<()> {
    todo!()
}
pub fn empty_list(i: Tokens) -> Result<()> {
    todo!()
}
pub fn colon(i: Tokens) -> Result<()> {
    todo!()
}
pub fn double_colon(i: Tokens) -> Result<()> {
    todo!()
}
pub fn semicolon(i: Tokens) -> Result<()> {
    todo!()
}
pub fn eq(i: Tokens) -> Result<()> {
    todo!()
}

pub fn open_paren(i: Tokens) -> Result<()> {
    todo!()
}
pub fn close_paren(i: Tokens) -> Result<()> {
    todo!()
}
pub fn open_bracket(i: Tokens) -> Result<()> {
    todo!()
}
pub fn close_bracket(i: Tokens) -> Result<()> {
    todo!()
}
pub fn open_squiggly(i: Tokens) -> Result<()> {
    todo!()
}
pub fn close_squiggly(i: Tokens) -> Result<()> {
    todo!()
}

pub fn comma(i: Tokens) -> Result<()> {
    todo!()
}

pub fn operator(i: Tokens) -> Result<Operator> {
    todo!()
}

