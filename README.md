# 意味論
- 静的スコープ
- 型は変数ではなく値に対して結合する(潜在的という)
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
(cdr '(a . b))  -> b
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
