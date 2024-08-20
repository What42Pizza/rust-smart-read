use std::{collections::VecDeque, mem::MaybeUninit};



pub fn array_to_single_element<T, const LEN: usize>(mut buffer: [T; LEN], index: usize) -> T {
	unsafe {
		let mut chosen_item = MaybeUninit::uninit().assume_init();
		std::ptr::copy_nonoverlapping(&buffer[index] as *const T, &mut chosen_item as *mut T, 1);
		for (i, item) in buffer.iter_mut().enumerate() {
			if i == index {continue;}
			std::ptr::drop_in_place(item as *mut T);
		}
		std::mem::forget(buffer); // buffer is on stack, no need to dealloc
		chosen_item
	}
}

pub fn vec_to_single_element<T>(mut buffer: Vec<T>, index: usize) -> T {
	unsafe {
		let mut chosen_item = MaybeUninit::uninit().assume_init();
		std::ptr::copy_nonoverlapping(&buffer[index] as *const T, &mut chosen_item as *mut T, 1);
		for (i, item) in buffer.iter_mut().enumerate() {
			if i == index {continue;}
			std::ptr::drop_in_place(item as *mut T);
		}
		buffer.set_len(0); // do this instead of forget() so that the underlying buffer is deallocated by official code
		chosen_item
	}
	//buffer.swap_remove(index) // safe but boring
}

pub fn vec_deque_to_single_element<T>(buffer: VecDeque<T>, index: usize) -> T {
	vec_to_single_element(buffer.into(), index)
	//buffer.swap_remove_back(index).unwrap() // safe but boring
	//unsafe {
	//	let mut chosen_item = MaybeUninit::uninit().assume_init();
	//	std::ptr::copy(&buffer[index] as *const T, &mut chosen_item as *mut T, 1);
	//	for (i, item) in buffer.iter_mut().enumerate() {
	//		if i == index {continue;}
	//		std::ptr::drop_in_place(item as *mut T);
	//	}
	//	//std::mem::forget(buffer); // doesn't dealloc underlying buffer
	//	//buffer.set_len(0); // function doesn't exist
	//	chosen_item
	//}
}
