fn main() {  // 프로그램의 시작점(진입 함수). 실행하면 가장 먼저 호출됨.
  
    // std::mem::size_of::<T>()는 타입 T가 메모리에서 차지하는 '바이트 수'를 알려줌
    println!("Size of a char: {}", std::mem::size_of::<char>()); // Rust의 char는 '유니코드 스칼라 값' 1개 = 항상 4바이트(32비트)

    // 문자열 리터럴은 UTF-8로 저장됨. .len()은 '문자 개수'가 아니라 '바이트 길이'를 반환함
    println!("Size of string containing 'a': {}", "a".len());   // "a"는 ASCII → 1바이트
    println!("Size of string containing 'ß': {}", "ß".len());   // 'ß'(U+00DF) → UTF-8에서 2바이트
    println!("Size of string containing '国': {}", "国".len()); // '国'(U+56FD) → UTF-8에서 3바이트
    println!("Size of string containing '𓅱': {}", "𓅱".len()); // 이집트 상형문자(보충 평면) → UTF-8에서 4바이트

    // &str(문자열 슬라이스) 리터럴. 프로그램 전체 동안 살아있는 '불변' 문자열
    let slice = "Hello!";
    println!("Slice is {} bytes.", slice.len()); // "Hello!"는 6글자 전부 ASCII → 6바이트

    // 한글은 UTF-8에서 보통 글자당 3바이트
    let slice2 = "안녕!"; // '안'(3바이트) + '녕'(3바이트) + '!'(1바이트) = 총 7바이트
    println!("Slice2 is {} bytes.", slice2.len()); // 7 출력
}
