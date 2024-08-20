use std::{collections::VecDeque, mem::MaybeUninit};



// These functions consume a buffer (array, Vec, VecDeque) and return one of
// its elements with minimal(?) copying.

// I've tried using std::ptr::read(), but that doesn't seem to work because
// it's not actually creating a copy of the data, meaning the data immediately
// becomes invalid.

pub fn array_to_single_element<T, const LEN: usize>(mut input: [T; LEN], index: usize) -> T {
	unsafe {
		let mut chosen_item = MaybeUninit::uninit().assume_init();
		std::ptr::copy_nonoverlapping(&input[index] as *const T, &mut chosen_item as *mut T, 1);
		for (i, item) in input.iter_mut().enumerate() {
			if i == index {continue;}
			std::ptr::drop_in_place(item as *mut T);
		}
		std::mem::forget(input); // arrays are on the stack, no need to dealloc
		chosen_item
	}
}

pub fn vec_to_single_element<T>(mut input: Vec<T>, index: usize) -> T {
	//input.swap_remove(index) // safe, but boring (and very slightly inefficient)
	unsafe {
		let mut chosen_item = MaybeUninit::uninit().assume_init();
		std::ptr::copy_nonoverlapping(&input[index] as *const T, &mut chosen_item as *mut T, 1);
		for (i, item) in input.iter_mut().enumerate() {
			if i == index {continue;}
			std::ptr::drop_in_place(item as *mut T);
		}
		input.set_len(0); // we want the underlying buffer to be deallocated by official code
		chosen_item
	}
}

pub fn vec_deque_to_single_element<T>(mut input: VecDeque<T>, index: usize) -> T {
	//vec_to_single_element(input.into(), index) // input.into::<Vec>() is probably more inefficient than swap_remove_back()
	input.swap_remove_back(index).unwrap() // safe, but boring (and very slightly inefficient)
	//unsafe {
	//	let mut chosen_item = MaybeUninit::uninit().assume_init();
	//	std::ptr::copy(&input[index] as *const T, &mut chosen_item as *mut T, 1);
	//	for (i, item) in input.iter_mut().enumerate() {
	//		if i == index {continue;}
	//		std::ptr::drop_in_place(item as *mut T);
	//	}
	//	//std::mem::forget(input); // doesn't dealloc underlying buffer
	//	//input.set_len(0); // function doesn't exist
	//	chosen_item
	//}
}
