fn main() {                             // 프로그램이 시작되는 진입점(메인 함수)

    // let my_float = 5.;               
    // (예시) 리터럴에 소수점(.)이 있으므로 '실수형'으로 추론됨 (기본 f64). 
    //       5.0처럼 0을 써도 되고, 5. 처럼 점만 있어도 실수로 인식됨. 
    // :contentReference[oaicite:1]{index=1}

    // let my_float: f64 = 5.0;          
    // (예시) 타입을 직접 지정: f64(64비트 실수). 
    // Rust는 실수 기본값으로 f64를 선호함. :contentReference[oaicite:2]{index=2}
    
    // let my_other_float: f32 = 8.5;    
    // (예시) f32(32비트 실수). f64와는 '서로 다른 타입'임.
    
    // let third_float = my_float +      
    // (예시) 여기까지 해석하면 'f64 + (뭔가)' → Rust는 '다음 항도 f64'일 거라 기대함.
    
    // let third_float = my_float + my_other_float;  
    // (예시) ⚠️ 컴파일 에러: 'expected f64, found f32'
    // f64와 f32는 바로 더할 수 없음 → 형을 맞춰야 함. 
    // :contentReference[oaicite:3]{index=3}

    let my_number = 9.6;                // 타입 표기를 생략했으므로 기본 실수형 f64로 추론됨. (실수 기본: f64) :contentReference[oaicite:4]{index=4}
    let other_number = 9;               // 정수 리터럴이므로 i32로 추론됨. (정수 기본: i32) :contentReference[oaicite:5]{index=5}

    println!("{}", my_number + other_number as f64 ); 
    // i32 → f64로 '형 변환(cast)' 후 더함.
    // 결과: 18.6
    // 포인트: 서로 다른 타입은 직접 연산 불가 → 한쪽을 'as'로 맞춰주기. 
    // :contentReference[oaicite:6]{index=6}
}
