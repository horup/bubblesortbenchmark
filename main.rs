use std::time::Instant;
 
fn bubble_sort_optimized(arr: &mut [i32]) {
    let len = arr.len();
    for j in 0..len {
        for i in 0..len {
            if arr[i] < arr[j] {
                arr.swap(i, j);
            }
        }
    }
}

fn main() {
	let mut array = [0_i32; 50 * 1000];
	let len = array.len();
	for i in 1..len {
		array[i] = len as i32 - i as i32;
	}
	
	let rerun = 10;
	let start = Instant::now();
	for _q in 0..rerun {
		bubble_sort_optimized(&mut array);
	}
	let duration = start.elapsed();
	
    println!("Took: {:?} to sort {:?} elements {:?} times", duration, array.len(), rerun); 
}