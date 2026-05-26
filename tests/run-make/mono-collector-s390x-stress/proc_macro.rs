extern crate proc_macro;

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

#[proc_macro]
pub fn generate_roots(input: TokenStream) -> TokenStream {
    let count = parse_count(input);
    let mut output = TokenStream::new();

    for index in 0..count {
        output.extend(function_tokens(index));
    }

    output.extend(main_tokens(count));
    output
}

fn parse_count(input: TokenStream) -> usize {
    let mut iter = input.into_iter();
    let Some(TokenTree::Literal(literal)) = iter.next() else {
        panic!("expected literal count");
    };
    literal.to_string().parse::<usize>().unwrap()
}

fn function_tokens(index: usize) -> TokenStream {
    let name = Ident::new(&format!("generated_root_{index}"), Span::call_site());
    let helper = Ident::new(&format!("helper_root_{index}"), Span::call_site());

    let mut tokens = TokenStream::new();
    tokens.extend([
        ident("pub"),
        ident("fn"),
        TokenTree::Ident(name),
        group(
            Delimiter::Parenthesis,
            [
                TokenTree::Ident(Ident::new("seed", Span::call_site())),
                punct(':', Spacing::Alone),
                ident("u64"),
            ],
        ),
        punct('-', Spacing::Joint),
        punct('>', Spacing::Alone),
        ident("u64"),
    ]);

    let body = quote_body(helper, index as u64 + 1);
    tokens.extend([group(Delimiter::Brace, body)]);
    tokens
}

fn main_tokens(count: usize) -> TokenStream {
    let mut body = TokenStream::new();
    body.extend([
        ident("let"),
        ident("mut"),
        TokenTree::Ident(Ident::new("sum", Span::call_site())),
        punct('=', Spacing::Alone),
        Literal::u64_unsuffixed(0).into(),
        punct(';', Spacing::Alone),
    ]);

    for index in 0..count {
        let name = Ident::new(&format!("generated_root_{index}"), Span::call_site());
        body.extend([
            TokenTree::Ident(Ident::new("sum", Span::call_site())),
            punct('=', Spacing::Alone),
            TokenTree::Ident(Ident::new("sum", Span::call_site())),
            punct('^', Spacing::Alone),
            TokenTree::Ident(name),
            group(Delimiter::Parenthesis, [Literal::u64_unsuffixed(index as u64 + 1).into()]),
            punct(';', Spacing::Alone),
        ]);
    }

    body.extend([
        ident("if"),
        TokenTree::Ident(Ident::new("sum", Span::call_site())),
        punct('=', Spacing::Joint),
        punct('=', Spacing::Alone),
        Literal::u64_unsuffixed(0).into(),
        group(
            Delimiter::Brace,
            [
                ident("std"),
                punct(':', Spacing::Joint),
                punct(':', Spacing::Alone),
                ident("process"),
                punct(':', Spacing::Joint),
                punct(':', Spacing::Alone),
                ident("exit"),
                group(Delimiter::Parenthesis, [Literal::i32_unsuffixed(1).into()]),
                punct(';', Spacing::Alone),
            ],
        ),
    ]);

    let mut tokens = TokenStream::new();
    tokens.extend([
        ident("fn"),
        ident("main"),
        group(Delimiter::Parenthesis, []),
        group(Delimiter::Brace, body),
    ]);
    tokens
}

fn quote_body(helper: Ident, salt: u64) -> TokenStream {
    let mut body = TokenStream::new();
    body.extend([
        ident("let"),
        ident("mut"),
        TokenTree::Ident(Ident::new("value", Span::call_site())),
        punct('=', Spacing::Alone),
        TokenTree::Ident(Ident::new("seed", Span::call_site())),
        punct(';', Spacing::Alone),
    ]);

    for depth in 0..8 {
        body.extend([
            TokenTree::Ident(Ident::new("value", Span::call_site())),
            punct('=', Spacing::Alone),
            TokenTree::Ident(helper.clone()),
            group(
                Delimiter::Parenthesis,
                [
                    TokenTree::Ident(Ident::new("value", Span::call_site())),
                    punct(',', Spacing::Alone),
                    Literal::u64_unsuffixed(salt + depth).into(),
                ],
            ),
            punct(';', Spacing::Alone),
        ]);
    }

    body.extend([TokenTree::Ident(Ident::new("value", Span::call_site()))]);
    body
}

fn ident(name: &str) -> TokenTree {
    TokenTree::Ident(Ident::new(name, Span::call_site()))
}

fn punct(ch: char, spacing: Spacing) -> TokenTree {
    TokenTree::Punct(Punct::new(ch, spacing))
}

fn group<I>(delimiter: Delimiter, tokens: I) -> TokenTree
where
    I: IntoIterator<Item = TokenTree>,
{
    let mut stream = TokenStream::new();
    stream.extend(tokens);
    TokenTree::Group(Group::new(delimiter, stream))
}
