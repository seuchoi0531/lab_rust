02-05
===
다른 언어에서의 `switch`와 유사한 `match` 키워드를 이용해서 Rust만의 독창적인 조건문 사용

`match` 구문의 형식은 아래와 같음
```
match [변수] {
  [조건] => {
    // ...
  }
  _ => {
    // ...
  }
}
```

여기서 `_`는 `switch`문의 `default`와 같은 역할이고, `[조건]`에는 한 가지 값, 여러 개의 값, 범위를 사용할 수 있음
```
match x {
  0 => {
    // ...
  }
  1 | 2 => {
    // ...
  }
  3..=9 => {
    // ...
  }
  _ => {
    // ...
  }
}
```

`x`와 조건을 비교했을 때 `true`면 해당 코드를 실행하게 됨
`switch`와 다른 점은 `break;`가 없어도 한 번 조건과의 비교에서 `true`였으면 해당 코드만 실행하고 `match` 문이 종료됨

찾은 숫자를 변수로 *바인딩*할 수 있음
```
match x % 2 {
  evenrest @ 0 => {
    println!("x % 2 = {}", evenrest);
  }
  oddrest @ _ => {
    println!("x % 2 = {}", oddrest);
  }
}
```
https://tourofrust.com/18_ko.html