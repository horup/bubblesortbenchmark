use std::time::Instant;
 
fn bubble_sort_optimized(arr: &mut [i32]) {
     let mut new_len: usize;
    let mut len = arr.len();
    loop {
        new_len = 0;
        for i in 1..len {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                new_len = i;
            }
        }
        if new_len == 0 {
            break;
        }
        len = new_len;
    }
}

fn main() {
	let mut array = [0_i32; 50 * 1000];
	let len = array.len();
	for i in 1..len {
		array[i] = i as i32;
	}
	
	let rerun = 10;
	let start = Instant::now();
	for _q in 1..rerun {
		bubble_sort_optimized(&mut array);
	}
	let duration = start.elapsed();
	
    println!("Took: {:?} to sort {:?} elements {:?} times", duration, array.len(), rerun); 
}