//! 正規表現の式をパースするための型・関数  
//! 式をパースして、抽象構文木(AST)に変換する。  
//! "ab+c*(def|ghi)"" が入力された場合、以下の AST に変換する  
//! 
//! ```text
//! Seq(
//!     Char(a),
//!     Plus(Char(b)),
//!     Star(Char(c)),
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
    let mut seq: Vec<AST> = Vec::new(); // 現在のコンテキスト
    let mut seq_or: Vec<AST> = Vec::new(); // Orのコンテキスト
    let mut stack: Vec<(Vec<AST>, Vec<AST>)> = Vec::new(); // コンテキストを一時的に退避させるスタック
    let mut is_escape: bool = false; // エスケープ文字を処理中かどうか

    for c in pattern.chars() {
        if is_escape {
            is_escape = false;
            seq.push(parse_escape(c));
            continue;
        }
        match c {
            '+' | '*' | '?' => {
                let prev_ast: AST = seq.pop().unwrap();
                let ast: AST = parse_qualifier(c, prev_ast);
                seq.push(ast);
            }
            '|' => {
                seq_or.push(AST::Seq(seq));
                seq = Vec::new();
            }
            '(' => {
                stack.push((seq, seq_or));
                seq = Vec::new();
                seq_or = Vec::new();
            }
            ')' => {
                let (mut prev, prev_or) = stack.pop().unwrap();

                if !seq.is_empty() {
                    seq_or.push(AST::Seq(seq));
                }
                prev.push(fold_or(seq_or));

                seq = prev;
                seq_or = prev_or;
            }
            '\\' => is_escape = true,
            _ => seq.push(AST::Char(c))
        };
    }

    // stack が空ではない = 閉じカッコが足りない
    if !stack.is_empty() {
        panic!()
    }

    if !seq.is_empty() {
        seq_or.push(AST::Seq(seq));
    }
    fold_or(seq_or)
}

// --- テストコード ---


#[cfg(test)]
mod tests {
    use crate::parser::{parse, AST};

    use super::parse_qualifier;

    #[test]
    fn test_escape() {
        assert_eq!(
            parse("\\*"),
            AST::Seq(vec![AST::Char('*')])
        );
    }

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