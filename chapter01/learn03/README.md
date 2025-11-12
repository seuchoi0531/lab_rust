## 01-03 
Rust에는 값의 변경 여부가 굉장히 중요함

변경 가능한*(mutable)* 값은 컴파일러가 변수에 값을 할당하는 것을 허용함
변경 불가능한*(immutable)* 값은 컴파일러가 변수의 값을 읽는 것만 허용함

변경 가능한 값은 `mut` 키워드로 표시함
```
let mut x = 42;
```

하지만 기본적으로 변수가 선언됐을 때 재할당 가능하므로, `mut` 키워드는 없어도 됨
`mut` 키워드를 사용하게 되면 warning: variable does not need to be mutable 발생

https://tourofrust.com/04_ko.html