fn main() {
    let vec0 = Vec::new();
    // START
    let vec1 = fill_vec(vec0);
    // END
    print_vec("vec1", &vec1);
    vec1.push(88);

    let vec0 = Vec::new();
    // START
    let mut vec1 = fill_vec(vec0);
    // END
    print_vec("vec0", &vec0);
    vec1.push(88);
    print_vec("vec1", &vec1);

    let vec0 = Vec::new();
    let mut vec1 = fill_vec2(&vec0);
    print_vec("vec0", &vec0);
    vec1.push(88);
    print_vec("vec1", &vec1);

    let mut vec0 = Vec::new();
    fill_vec3(&mut vec0);
    print_vec("vec0", &vec0);

    let vec0 = Vec::new();
    let mut vec1 = fill_vec4(vec0);
    print_vec("vec1", &vec1);
    vec1.push(88);
    print_vec("vec1", &vec1);

    let mut vec1 = fill_vec5();
    print_vec("vec1", &vec1);
    vec1.push(88);
    print_vec("vec1", &vec1);
}

fn print_vec(name: &str, vec: &Vec<i32>) {
    println!("{} has length {} content `{:?}`", name, vec.len(), vec);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
    vec.push(22);
    vec
}

// START
fn fill_vec2(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
    // END
    vec.push(22);
    vec
}

// START
fn fill_vec3(vec: Vec<i32>) -> Vec<i32> {
    // END
    vec.push(22);
}

// START
fn fill_vec4(vec: Vec<i32>) -> Vec<i32> {
    // END
    vec.push(22);
    vec
}

fn fill_vec5() -> Vec<i32> {
    // START
    let mut vec = vec;
    // END
    vec.push(22);
    vec
}
