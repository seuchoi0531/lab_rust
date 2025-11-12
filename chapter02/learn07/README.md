02-07
===
Rust에서 `{}`으로 감싸져 있는 부분을 *"블록"*이라고 함
블록에서 무언가를 반환할 때 꼭 `return` 키워드를 사용하지 않아도 반환할 수 있음
구문에 `;`가 없으면 그 값을 블록의 반환 값으로 간주함
따라서, `if` 문, `match` 문, 블록 구문, 함수의 반환을 아래 코드처럼 표현할 수 있음

- `if` 문 예시. 삼항 연산자처럼 표현 가능
```
let a = 50;
let isOdd = if a % 2 == 0 { false } else { true };
println!("{}", isOdd); // false
```

- `match` 문 예시. `,`로 각 조건을 구분
```
let x = 17;
let rst = match y {
    0..=10 => "10 이하",
    11.. => "10 초과",
    _ => "그럴 순 없음",
};
println!("{}", rst); // 10 초과
```

- 블록 예시.
```
let v = {
    let a = 4;
    let b = 11;
    a + b
};
println!("{}", v); // 15
```

- 함수 예시.
```
fn add4(a: i32) -> i32 {
    a + 4
}
```

https://tourofrust.com/20_ko.html