
//use std::ops::Deref;
use structs::slice::CmdlineSlice;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use iter::two::CmdlineTwoIter;
use iter::one::CmdlineOneIter;
use iter::CmdlineIter;
use Cmdline;
use CmdlineErr;

#[derive(Debug, Hash, Clone, Eq, PartialEq, PartialOrd)]
pub struct CmdlineBuf(Vec<u8>);

impl CmdlineBuf {
	///Creates cmdline from `Vec`.
	#[inline]
	pub fn array(array: Vec<u8>) -> Self {
		CmdlineBuf(array)
	}
	
	pub fn reopen< P: AsRef<Path> >(&mut self, path: P) -> Result<(), CmdlineErr> {
		match File::open(path) {
			Ok(mut file) => {
				self.0.clear();
				match file.read_to_end(&mut self.0) {
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
		CmdlineIter::new(&mut self.0)
	}
	
	#[inline]
	fn iter_one<'i>(&'i mut self) -> CmdlineOneIter<'i> {
		CmdlineOneIter::new(&mut self.0)
	}
	
	#[inline]
	fn iter_two<'i>(&'i mut self) -> CmdlineTwoIter<'i> {
		CmdlineTwoIter::new(&mut self.0)
	}
}

impl AsRef<[u8]> for CmdlineBuf {
	#[inline]
	fn as_ref(&self) -> &[u8] {
		self.0.as_slice()
	}
}
impl AsRef<Vec<u8>> for CmdlineBuf {
	#[inline]
	fn as_ref(&self) -> &Vec<u8> {
		&self.0
	}
}
impl AsRef<CmdlineBuf> for CmdlineBuf {
	#[inline]
	fn as_ref(&self) -> &CmdlineBuf {
		&self
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
impl<'a> From<CmdlineSlice<'a>> for CmdlineBuf {
	#[inline]
	fn from(slice: CmdlineSlice<'a>) -> CmdlineBuf {
		slice.to_buf()
	}
}


impl Into<Vec<u8>> for CmdlineBuf {
	fn into(self) -> Vec<u8> {
		self.0
	}
}