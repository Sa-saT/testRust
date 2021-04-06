struct SampleA{
    x: i32,
}

impl SampleA{
    fn new(x: i32) -> SampleA{
        SampleA{x: x}
    }
    fn inc(&self) -> i32{
        self.x +1
    }
    fn add(&self, x: i32) -> i32{
        self.x + x
    }
}

fn main() {
    let a = SampleA::new(10);
    println!("inc is {}", a.inc());
    println!("add is {}", a.add(20));

    //型宣言しなくても(型推論)使える
    let name : &str = "takumi";
    let age : i32 = 30;
    let t = (name, age);
    println!("name {} age {}", t.0, t.1);

    let mut s = String::new();

    let mut av = vec![1, 2, 3, 4, 5, 6, 7];
    print!("{:?}", av);
    println!("");
    let a_sum = vec_param(&av);

    println!("a_sum is {}", a_sum);

    //let cv = &av ;

    vec_change(&mut av);
    println!("av *10 in a row!");
    //参照(アドレス)でもいける
    for i in &av{
        print!("{} ", *i);
    }
    print!("x: ");
    for i in &av{
        let x: i32 = *i;
        print!("{} ", x);
    }
    println!(" /end");

    println!("bv in a row!");
    let bv = vec_return(10);
    for (i, x) in bv.iter().enumerate(){
        print!("{}:{} ", i, x);
    }
    println!("/ end");

    println!("数値, {}", add(4,3));
    println!("{} は {}才", name, age);

    str_test(&mut s);

    println!("{}", s);
    loop_t();
    abc();
}

fn abc(){
    let num = 10;
    let add_one = |x| { num + x };
    let add_two = |x, y| { x + y };

    println!("add_one is {}", add_one(2));
    println!("add_two is {}", add_two(2,3));
}

// ベクターの中身を変える関数
fn vec_change(v: &mut Vec<i32>){
    for i in v{
        *i = *i * 10;
    }
}
// ベクターを返す関数
fn vec_return(max: i32) -> Vec<i32>{
    let mut vv = Vec::new();
    for i in 2..max{
        if i == 5{
            break;
        }
        else if i % 2 == 0{
            continue;
        }
        vv.push(i);
    }
    vv
}

fn loop_t(){
    let mut j = 1;
    loop{
        if j >= 5{
            break;
        }
        j += j;
        print!("{} ",j);
    }
    println!(" {}", j);
}
// ベクターを受け取る関数
fn vec_param(v: &Vec<i32>) -> i32{
    let mut sum = 0;
    for i in v {
        sum += i;
    }
    sum
}

//"fn"は関数宣言,function
//"->"は関数の戻り値
fn add(x : i32, y : i32) -> i32{
    x + y
}

fn str_test(s: &mut String) {
    let s1 = String::from("hello");
    let s2 = String::from("world");
    *s = format!("{}, {}", s1, s2);
}
