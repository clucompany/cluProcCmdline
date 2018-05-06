

use std::slice::Iter;

#[derive(Debug, Clone)]
pub struct CmdlineOneIter<'i> {
	array: &'i [u8],
	iter: Iter<'i, u8>,
	
	s_n: usize,
	s_e: usize,
}

impl<'i> CmdlineOneIter<'i> {
	#[inline]
	pub fn new(array: &'i [u8]) -> CmdlineOneIter<'i> {
		CmdlineOneIter {
			array: array,
			iter: array.iter(),
			
			s_n: 0,
			s_e: 0,
		}
	}
	#[inline]
	pub fn clear(&mut self) {
		self.s_n = 0;
		self.s_e = 0;
		self.iter = self.array.iter();
	}
}

impl<'i> Iterator for CmdlineOneIter<'i> {
	type Item = &'i [u8];
	
	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		'begin: while let Some(a) = self.iter.next() {
			match *a {
				b'=' => {	
					self.s_e += 1;			
					while let Some(a) = self.iter.next() {
						match *a {
							10u8 | b' ' => {
								break;
							},
							_ => {
								self.s_e += 1;
							},
						}
					}

					/*self.s_e += 1;
					self.s_n = self.s_e;*/
					
					self.s_e += 1;
					self.s_n = self.s_e;
					
					continue 'begin;
				},
				
				10u8 | b' ' => {
					break;
				},
				_ => {
					self.s_e += 1;
				},
			}
		}
		
		if self.s_n != self.s_e {
			let array_0 = unsafe { self.array.get_unchecked(self.s_n .. self.s_e) };
					
			self.s_e += 1;
			self.s_n = self.s_e;
					
					
			return Some(array_0);
					
		}
		return None;
	}
}
