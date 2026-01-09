use std::cell::Cell;

fn retain_even(nums: &mut Vec<i32>) {
    let slice: &[Cell<i32>] = Cell::from_mut(&mut nums[..]).as_slice_of_cells();

    let mut i = 0;
    for num in slice.iter().filter(|num| num.get() % 2 == 0) {
        slice[i].set(num.get());
        i += 1;
    }

    nums.truncate(i);
}

fn main() {
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    retain_even(&mut vec);

    for n in vec {
        println!("{}", n);
    }
}
