02-06
===
`loop` 문에서 `break`와 동시에 값을 반환할 수 있음
```
let x = 1;
let v = loop {
  x += 1;
  if x == 13 {
    break "13 발견";
  }
}
println!("{}", v); // 13
```

https://tourofrust.com/19_ko.html