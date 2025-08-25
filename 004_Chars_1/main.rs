fn main() {                                      // 프로그램의 시작점: main 함수

    // println!('Hello, world!');                // (잘못된 예시) 문자열에 작은따옴표 사용 → 컴파일 에러
    // string은 무조건 ""으로 ''으로 입력하면 에러 발생
    // help: if you meant to write a string literal, use double quotes
    // error[E0762]: unterminated character literal

    println!("Hello, world!");                   // (정상) 문자열은 큰따옴표(")를 사용해 출력

    let first_letter = 'A';                      // char 리터럴: 작은따옴표(') 사용, 단일 문자 저장
    // char 타입은 Rust에서 문자(Unicode 단일 문자)로 이모지도 됨
    // char size = 4 bytes
    println!("문자: {}", first_letter);          // 포맷 문자열에 {}로 값 삽입하여 출력

    // 다양한 유니코드 문자 저장 가능
    let korean = '한';                           // 한글 1글자 (유니코드)
    let chinese = '字';                          // 한자 1글자 (유니코드)
    let emoji = '😊';                            // 이모지 1개 (유니코드)

    println!("한글: {}, 한자: {}, 이모지: {}",  // 여러 값을 한 번에 출력
             korean, chinese, emoji);

    // casting = simple type change using 'as'
    let my_number = 'a' as u8;                   // 'a'의 유니코드(ASCII) 값(97)으로 캐스팅
    println!("Hello, world! My number is {}",    // 숫자값 출력
             my_number);

    /*
    // 결과 코드 :
    Hello, world!
    문자: A
    한글: 한, 한자: 字, 이모지: 😊
    Hello, world! My number is 97
    */
}
