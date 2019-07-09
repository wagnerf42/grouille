//! Some more advanced iterators.
use std::slice::Windows;
use streaming_iterator::StreamingIterator;

/// Returns an iterator over windows of given size (wrapping back to the beginning).
pub trait GrouilleSlice {
    /// Items on which we iterate. They need to implement Clone.
    type Item: Clone;
    /// Loop on windows of given size. At the end, we wrap at the beginning.
    fn wrapping_windows<'a>(&'a self, block_size: usize)
        -> WrappingWindowsIterator<'a, Self::Item>;
}

impl<T: Clone> GrouilleSlice for [T] {
    type Item = T;
    fn wrapping_windows<'a>(
        &'a self,
        block_size: usize,
    ) -> WrappingWindowsIterator<'a, Self::Item> {
        wrapping_windows_fn(self, block_size)
    }
}

/// Iterator returned by `wrapping_windows`.
pub struct WrappingWindowsIterator<'a, T: 'a> {
    initial_window_iterator: Windows<'a, T>,
    remaining_elements: Vec<T>,
    remaining_index: usize,
    block_size: usize,
    current_win: Option<&'a [T]>,
}

/// iterate over slice with a windows of given block size which wraps back at the beginning
fn wrapping_windows_fn<'a, T: Clone + 'a>(
    slice: &'a [T],
    block_size: usize,
) -> WrappingWindowsIterator<'a, T> {
    let mut windows = slice.windows(block_size);
    let current_win = windows.next();
    WrappingWindowsIterator {
        initial_window_iterator: windows,
        remaining_elements: slice[(slice.len() - block_size + 1)..]
            .iter()
            .chain(slice[..block_size].iter())
            .cloned()
            .collect::<Vec<T>>(),
        remaining_index: 0,
        block_size,
        current_win,
    }
}

impl<'a, T: 'a> StreamingIterator for WrappingWindowsIterator<'a, T> {
    type Item = [T];
    fn advance(&mut self) {
        if self.current_win.is_some() {
            self.current_win = self.initial_window_iterator.next();
        } else {
            self.remaining_index += 1;
        }
    }
    fn get(&self) -> Option<&Self::Item> {
        if self.current_win.is_none() {
            if self.remaining_index + self.block_size <= self.remaining_elements.len() {
                Some(
                    &self.remaining_elements
                        [self.remaining_index..(self.remaining_index + self.block_size)],
                )
            } else {
                None
            }
        } else {
            self.current_win
        }
    }
}
