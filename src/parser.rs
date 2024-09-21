//! 正規表現の式をパースするための型・関数  
//! 式をパースして、抽象構文木(AST)に変換する。  
//! "abc(def|ghi)"" が入力された場合、以下の AST に変換する  
//! 
//! ```text
//! Seq(
//!     Char(a),
//!     Char(b),
//!     Char(c),
//!     Or(
//!         Seq(
//!             Char(d),
//!             Char(e),
//!             Char(f)
//!         ),
//!         Seq(
//!             Char(g),
//!             Char(h),
//!             Char(i)
//!         )
//!     )
//! )
//! ```

use std::mem::take;

/// AST の型
#[derive(Debug, PartialEq)]
pub enum AST {
    Char(char),             // 通常の文字に対応する型
    Plus(Box<AST>),         // '+'に対応する型
    Star(Box<AST>),         // '*'に対応する型
    Question(Box<AST>),     // '?'に対応する型
    Or(Box<AST>, Box<AST>), // '|'に対応する型
    Seq(Vec<AST>),          // 連結に対応する型
}

/// エスケープ文字から AST を生成
fn parse_escape(c: char) -> AST {
    match c {
        '\\' | '(' | ')' | '|' | '+' | '*' | '?'=> AST::Char(c),
        _ => panic!(),
    }
}

/// `+`,`*`,`?`から AST を生成
fn parse_qualifier(c: char, prev: AST) -> AST{
    match c {
        '+' => AST::Plus(Box::new(prev)),
        '*' => AST::Star(Box::new(prev)),
        '?' => AST::Question(Box::new(prev)),
        _ => unreachable!()
    }
}

/// `|` を含む式から AST を生成
fn fold_or(mut seq_or: Vec<AST>) -> AST {
    if seq_or.len() > 1 {
        let mut ast: AST = seq_or.pop().unwrap();
        seq_or.reverse();
        for s in seq_or {
            ast = AST::Or(Box::new(s), Box::new(ast));
        }
        ast
    } else {
        seq_or.pop().unwrap()
    }
}

/// 式をパースし、ASTを生成
pub fn parse(pattern: &str) -> AST {
    let mut seq: Vec<AST> = Vec::new(); // ___
    let mut seq_or: Vec<AST> = Vec::new(); // ___
    let mut stack: Vec<(Vec<AST>, Vec<AST>)> = Vec::new(); // 処理中のASTを退避させる

    let mut is_escape: bool = false;

    for c in pattern.chars() {
        if is_escape {
            is_escape = false;
            seq.push(parse_escape(c));
        }
        match c {
            '+' | '*' | '?' => {
                let prev_ast: AST = seq.pop().unwrap();
                let ast: AST = parse_qualifier(c, prev_ast);
                seq.push(ast);
            },
            '(' => {
                let prev: Vec<AST> = take(&mut seq);
                let prev_or: Vec<AST> = take(&mut seq_or);
                stack.push((prev, prev_or));
            },
            ')' => {
                let (mut prev, prev_or) = stack.pop().unwrap();

                if !seq.is_empty() {
                    seq_or.push(AST::Seq(seq));
                }
                prev.push(fold_or(seq_or));

                seq = prev;
                seq_or = prev_or;
            }
            '|' => {
                let prev: Vec<AST> = take(&mut seq);
                seq_or.push(AST::Seq(prev));
            },
            '\\' => is_escape = true,
            _ => seq.push(AST::Char(c))
        };
    }
    // 閉じカッコが足りないエラー
    if !stack.is_empty() {
        panic!()
    }

    if !seq.is_empty() {
        seq_or.push(AST::Seq(seq));
    }

    fold_or(seq_or)
}

// --- test code ---


#[cfg(test)]
mod tests {
    use crate::parser::{AST, parse};

    use super::parse_qualifier;

    #[test]
    fn test_qualifier() {
        let plus_ast: AST = AST::Plus(Box::new(AST::Char('a')));
        assert_eq!(parse_qualifier('+', AST::Char('a')), plus_ast);

        let star_ast: AST = AST::Star(Box::new(AST::Char('a')));
        assert_eq!(parse_qualifier('*', AST::Char('a')), star_ast);

        let question_ast: AST = AST::Question(Box::new(AST::Char('a')));
        assert_eq!(parse_qualifier('?', AST::Char('a')), question_ast);
    }

    #[test]
    fn test_parse() {
        // "abc(def|ghi)" が入力されたケース
        let expect_ast: AST = AST::Seq(vec![
            AST::Char('a'), AST::Char('b'), AST::Char('c'),
            AST::Or(
                Box::new(AST::Seq(vec![AST::Char('d'), AST::Char('e'), AST::Char('f'),])),
                Box::new(AST::Seq(vec![AST::Char('g'), AST::Char('h'), AST::Char('i'),]))
            )
        ]);

        let actual_ast: AST = parse("abc(def|ghi)");
    
        assert_eq!(actual_ast, expect_ast);
    }
}