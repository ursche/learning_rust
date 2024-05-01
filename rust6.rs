//compound types (tuples and array)
//tuples
fn main() {
   let tup : (i32,u32,char) =( -5,6,'a');
   let (x,y,z) = tup;
   println!("{x}");
   println!("{y}");
  println!("{z}");


//only value needed

let x1 = tup.0;
let x2 = tup.1;
let x3 = tup.2;
println!("{x1}");
println!("{x2}");
println!("{x3}");

//array

   let arr = [1,2,3,4,5];
   let arr1 : [i32;5] = [1,2,3,4,5];

   let ar1 = arr[0];
   let ar2 = arr[3];

   println!("{ar1}");
   println!("{ar2}");

   println!("{:?}",arr);   //to print whole array

}
