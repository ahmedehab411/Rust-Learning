fn main() {
    /*let's say we need to find the largest number in 
    a vector let's write the code for doing that. */
    let v = vec![25,12,100,60,80];

    let max = find_max(v);

    println!("maximum number is: {}",max);

    /*now if we need to find the maximum in a new vector we will simply rewrite the code
    so we can simply extract the code into a function (find_max()) here now what happen
    if we need to work also with another data-types not only i32. we will simply create 
    a copy of the function to be able to handle different data-types. or  simply we can
    use templates.*/

    /*Generic types (templates) can also be used in structures and enums. */
    let p1 = Point{x:5.0, y:10.0};
    let p2 = Point{x:5, y:10};
    p1.x();
    p1.y();
    p2.x();
    //p2.y(); // not available

    /*Note that using generic types doesn't affect the performance as it duplicates the
    function , struct or enum during the compilation time. */
}

//PartialOrd and copy are traits will know about them in the next lesson.
fn find_max<T: PartialOrd + Copy>(v: Vec<T>) -> T {
    let mut max = v[0];
    for element in v {
        if element > max {
            max = element;
        }
    }
    max
}

struct Point<T> {
    x:T,
    y:T,
}

impl<U> Point<U> {
    fn x(&self) -> &U {
        &self.x
    }
}

impl Point<f64> {
    fn y(&self) -> f64 {
        self.y
    }
}