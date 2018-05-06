
//use std::ops::Deref;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use iter::two::CmdlineTwoIter;
use iter::one::CmdlineOneIter;
use iter::CmdlineIter;
use Cmdline;
use CmdlineErr;

#[derive(Debug, Hash, Clone, Eq, PartialEq, PartialOrd)]
pub struct CmdlineBuf {
	array: Vec<u8>,
}

impl CmdlineBuf {
	///Creates cmdline from `Vec`.
	#[inline]
	pub fn array(array: Vec<u8>) -> CmdlineBuf {
		CmdlineBuf {
			array: array,
		}
	}
	
	pub fn reopen< P: AsRef<Path> >(&mut self, path: P) -> Result<(), CmdlineErr> {
		match File::open(path) {
			Ok(mut file) => {
				self.array.clear();
				match file.read_to_end(&mut self.array) {
					Ok(size) => {
						if size == 0 {
							return Err(CmdlineErr::EmptyFile);
						}
						
						return Ok(());
					},
					Err(e) => return Err(CmdlineErr::ReadFile(e)),
				}
			},
			Err(e) => return Err(CmdlineErr::OpenFile(e)),
		}
	}
}
impl Cmdline for CmdlineBuf {
	#[inline]
	fn iter<'i>(&'i mut self) -> CmdlineIter<'i> {
		CmdlineIter::new(&mut self.array)
	}
	
	#[inline]
	fn iter_one<'i>(&'i mut self) -> CmdlineOneIter<'i> {
		CmdlineOneIter::new(&mut self.array)
	}
	
	#[inline]
	fn iter_two<'i>(&'i mut self) -> CmdlineTwoIter<'i> {
		CmdlineTwoIter::new(&mut self.array)
	}
}

impl AsRef<[u8]> for CmdlineBuf {
	#[inline]
	fn as_ref(&self) -> &[u8] {
		self.array.as_slice()
	}
}
impl AsRef<Vec<u8>> for CmdlineBuf {
	#[inline]
	fn as_ref(&self) -> &Vec<u8> {
		&self.array
	}
}


/*
impl Deref for CmdlineBuf {
	type Target = [u8];
	
	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.array.as_slice()
	}
}

*/


impl Into<Vec<u8>> for CmdlineBuf {
	#[inline]
	fn into(self) -> Vec<u8> {
		self.array
	}
}


impl From<Vec<u8>> for CmdlineBuf {
	#[inline]
	fn from(array: Vec<u8>) -> CmdlineBuf {
		CmdlineBuf::array(array)
	}
}
impl<'a> From<&'a [u8]> for CmdlineBuf {
	#[inline]
	fn from(array: &'a [u8]) -> CmdlineBuf {
		CmdlineBuf::array(array.to_vec())
	}
}
