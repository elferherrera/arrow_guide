use arrow::array::Array;
use arrow::array::Int32Array;
use arrow::compute::kernels::comparison::eq;

fn main() {
    let a = Int32Array::from(vec![6, 7, 8, 8, 10]);
    let b = Int32Array::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    println!("{:?}", a);
    println!("{:?}", b);

    let b_slice = b.slice(5, 5);
    println!("{:?}", b_slice);

    //let c = b_slice.as_any().downcast_ref().unwrap();
    //println!("{:?}", c);

    let d = eq(&b, &a).unwrap();

    assert_eq!(true, d.value(0));
    assert_eq!(true, d.value(1));
    assert_eq!(true, d.value(2));
    assert_eq!(false, d.value(3));
    assert_eq!(true, d.value(4));
}
