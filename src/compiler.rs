//! AST を命令列(Instruction)にコンパイルするための型・関数  
//! "ab(c|b)" が入力された場合、以下にコンパイルする
//! (左の数字はプログラムカウンタ)
//! 
//! ```text
//! 0 : Char(a)
//! 1 : Char(b)
//! 2 : Split 3, 5
//! 3 : Char(c)
//! 4 : Jump 6
//! 5 : Char(d)
//! 6 : Match
//! ```

use crate::parser::AST;

/// 命令列の型
#[derive(Debug, PartialEq)]
pub enum Instruction {
    Char(char),
    Match,
    Jump(usize),
    Split(usize, usize),
}

/// コンパイラの型
#[derive(Default, Debug)]
struct Compiler {
    p_counter: usize,
    instructions: Vec<Instruction>
}

impl Compiler {
    /// 入力された AST の型に応じた関数を実行する
    fn gen_expr(&mut self, ast: &AST) {
        match ast {
            AST::Char(c) => self.gen_char(*c),
            AST::Or(e1, e2) => self.gen_or(e1, e2),
            AST::Plus(ast) => self.gen_plus(ast),
            AST::Star(ast) => self.gen_star(ast),
            AST::Question(ast) => self.gen_question(ast),
            AST::Seq(v) => self.gen_seq(v),
        }
    }

    /// AST::Char 型に対応する Instruction を生成し、instructions に push する
    fn gen_char(&mut self, c: char) {
        let inst: Instruction = Instruction::Char(c);
        self.p_counter += 1;
        self.instructions.push(inst);
    }

    /// AST::Star 型に対応する Instruction を生成し、instructions に push する  
    /// a* 入力された場合、以下のような Instruction を生成する  
    /// 
    /// ```text
    /// 0 : split 1, 3
    /// 1 : Char(a)
    /// 2 : jump 0 
    /// 3 : ... 続き
    /// ```
    fn gen_star(&mut self, ast: &AST) {
        let split_count: usize = self.p_counter;

        // カウンタをインクリメントし、split を挿入する

        // 第二引数は、後に出てくる Jump のカウンタの数値を示すものであり、この時点では決まらないので仮の数値(ここでは 0 )を入れる
        // 仮の数値は、Jump を挿入した後に更新する
        self.p_counter += 1;
        self.instructions.push(Instruction::Split(self.p_counter, 0));

        // AST を再帰的に処理する
        self.gen_expr(ast);
        
        // カウンタをインクリメントし、Jump を挿入する
        self.p_counter += 1;
        self.instructions.push(Instruction::Jump(split_count));

        // 仮の数値としていた Split の第二引数を更新する
        if let Some(Instruction::Split(_, right)) = self.instructions.get_mut(split_count) {
            *right = self.p_counter;
        }
    }

    /// AST::Plus 型に対応する Instruction を生成し、instructions に push する  
    /// a+ 入力された場合、以下のような Instruction を生成する  
    /// 
    /// ```text
    /// 0 : Char(a)
    /// 1 : split 0, 2
    /// 2 : ... 続き
    /// ```
    fn gen_plus(&mut self, ast: &AST) {
        let left: usize = self.p_counter;
        // AST を再帰的に処理する
        self.gen_expr(ast);

        // カウンタをインクリメントし Split を挿入する
        self.p_counter += 1;
        self.instructions.push(Instruction::Split(left, self.p_counter));
    }

    /// AST::Question 型に対応する Instruction を生成し、instructions に push する  
    /// a? 入力された場合、以下のような Instruction を生成する  
    /// 
    /// ```text
    /// 0 : split 1, 2
    /// 1 : Char(a)
    /// 2 : ... 続き
    /// ```
    fn gen_question(&mut self, ast: &AST) {
        let split_count: usize = self.p_counter;
        // カウンタをインクリメントし、split を挿入する
        // 第二引数は、この時点では決まらないので仮の数値(ここでは 0 )を入れる
        // 仮の数値は、後に更新する
        self.p_counter += 1;
        self.instructions.push(Instruction::Split(self.p_counter, 0));

        // AST を再帰的に処理する
        self.gen_expr(ast);

        // 仮の数値としていた Split の第二引数を更新する
        if let Some(Instruction::Split(_, right)) = self.instructions.get_mut(split_count) {
            *right = self.p_counter;
        }
    }

    /// AST::Or 型に対応する Instruction を生成し、instructions に push する  
    /// a|b が入力された場合、以下のような Instruction を生成する。  
    /// 
    /// ```text
    /// 0 : split 1, 3
    /// 1 : Char(a)
    /// 2 : jump 4 
    /// 3 : Char(b)
    /// 4 : ... 続き
    /// ```
    fn gen_or(&mut self, expr1: &AST, expr2: &AST) {
        let split_counter: usize = self.p_counter;

        // カウンタをインクリメントし、split を挿入する
        // 第二引数は、expr2のコードの開始のカウンタを指定するため、この時点では決まらない
        // ここでは仮の数値(0)を入れて。数値は後で更新する
        self.p_counter += 1;
        self.instructions.push(Instruction::Split(self.p_counter, 0));

        // 1つ目の AST を再帰的に処理する
        self.gen_expr(expr1);

        let jump_counter: usize = self.p_counter;

        // カウンタをインクリメントし、split を挿入する。
        // 引数は、expr2 のコードの次のカウンタを指定するため、この時点では決まらない
        // ここでは仮の数値(0)を入れて。数値は後で更新する
        self.p_counter += 1;
        self.instructions.push(Instruction::Jump(0));

        // Splitの第二引数を更新する
        if let Some(Instruction::Split(_, right)) = self.instructions.get_mut(split_counter) {
            *right = self.p_counter;
        };

        // 2つ目の AST を再帰的に処理する
        self.gen_expr(expr2);

        // Jumpの引数を更新する
        if let Some(Instruction::Jump(arg)) = self.instructions.get_mut(jump_counter) {
            *arg = self.p_counter;
        }
    }

    /// AST::Seq 型に対応する Instruction を生成し、instructions に push する
    fn gen_seq(&mut self, vec:&Vec<AST>) {
        for ast in vec {
            self.gen_expr(ast)
        }
    }

    /// AST から Instruction を生成し、instructions に push する  
    /// 最後に Match を instructions に push する
    fn gen_code(&mut self, ast: &AST) {
        self.gen_expr(ast);
        self.instructions.push(Instruction::Match);
    }
}

/// コード生成を行う関数
pub fn compile(ast: &AST) -> Vec<Instruction> {
    let mut compiler: Compiler = Compiler::default();
    compiler.gen_code(ast);
    compiler.instructions
}

// ----- テストコード -----
