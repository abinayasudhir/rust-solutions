use num::integer::sqrt;

fn main() {
    let numbers = [36, 25, 49, 3, 64, 16, 9];
    let prime = get_prime(numbers);
}

fn get_prime(arr: [i32; 7]) -> i32 {

    let mut i = 0;
    'outer: loop {

        let mut n = 2;
        'inner: loop {

            if arr[i] % n == 0 {
                if arr[i] == 2 {
                    break 'outer;
                }
                i += 1;
                break;
            }

            if n >= sqrt(arr[i]) {
                break 'outer;
            }

            n += 1;
        }
    }
    println!("The first prime number in the array is {}.", arr[i]);
    arr[i]
}
