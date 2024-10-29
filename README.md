# 意味論(Semantics)
- 静的スコープ
- 型は変数ではなく値に対して結合する(型が潜在的という)
- 値はオブジェクトと呼ぶ
- オブジェクトは無限の寿命を持つが、今後使われないことが保証出来るなら記憶領域を再利用してよい(ガーベージコレクション)
- 実装は真に末尾再帰的でなければならない
    - 繰り返し計算が定数空間で実行出来る
- 手続きはそれ自体がオブジェクト
- 
# 基本演算
- cons
- car
- cdr
- define

```lisp
(cons 'a 'b)    -> (a . b)
(cons 'a '(b))  -> (a b)
(car '(a . b))  -> a
(cdr '(a . b))  -> b
(cdr '(a . b))  -> bけい
```

# 式
## 原始式(Primitive Expression)
- 変数参照(Variable Reference)
```scheme
(define x 28)
x ; -> 28
(* x 4) ; -> 112
```
- リテラル(Literal Expression)
```scheme
(quote a) ; -> a
(quote #(a b c)) ; -> #(a b c)
(quote (+ 1 2)) ; -> (+ 1 2)
```
- 手続き呼び出し(Procedure Call)
- 手続き(Procedure)
- 条件式(Conditional)
- 代入(Assignment)
## 派生式(Derived Expression)
