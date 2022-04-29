
fn main() {
    println!("Hello, world!");

    {
        // 所有権:原本 値:不変
        let a = 1;
        // a = 2; cannot assign twice to immutable variable

        // 所有権:原本 値:可変
        let mut b = 2;
        b = 3;

        // 所有権:仮   値:不変 (ポインタ的なもの?
        let c = &a;
        let d = &a;
        
        // 所有権:仮   値:可変
        let mut e = 10;
        let f = &mut e;
        *f = 20;
        // let g = &mut a; cannot borrow `a` as mutable, as it is not declared as mutable
        
        println!("a:{}, b:{}, c:{}, d:{}, f:{}",a, b, c, d, f);
    }
    
    {
        // スライス
        let a = [1,2,3,4,5];
        dbg!(a);
        let b = &a[1..3]; // [2,3]
        dbg!(b);

        // range
        let mut sum = 0;
        for i in 1..100 {
            sum += i;
        }
        dbg!(sum); // 4950

        let hello = "Hello World!";
        dbg!(&hello[2..10]); // "llo Worl"
    }

    {
        // 関数 (型の書き方がgoと似てる
        fn add(a:i32, b:i32) -> i32 {
            // return不要
            a + b
        }

        let rslt = add(10, 20);
        dbg!(rslt); // 30

        // 関数を引数として受け取る
        fn calc(func: Box<dyn Fn(i32, i32) -> i32>, a:i32, b:i32) -> i32 {
            func(a, b)
        }
        let rslt2 = calc(Box::new(add), 30, 40);
        dbg!(rslt2); // 70

        // 無名関数を渡す
        let rslt3 = calc(Box::new(|x, y|{x * y}), 7, 6);
        println!("{}", rslt3) // 42
    }

    {   
        // for式
        for i in 1..10 {
            println!("for {}", i);
        }

        // while式
        let mut i = 0;
        while i < 10 {
            println!("while {}", i);
            i += 1;
        }

        // loop式
        let mut num = 1;
        loop {
            // if else 式
            if num % 15 == 0 {
                println!("fizzbuzz")
            } else if num % 3 == 0 {
                println!("fizz")
            } else if num % 5 == 0 {
                println!("buzz")
            } else {
                println!("{}", num)
            }
            
            num += 1;
            if num > 30 {
                break
            }
        }

        let b = false;
        println!("{}",if b {"b is true"} else {"b is false"});
    }
}
