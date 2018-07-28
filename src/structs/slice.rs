
//use std::ops::Deref;
use structs::buf::CmdlineBuf;
use iter::one::CmdlineOneIter;
use iter::two::CmdlineTwoIter;
use iter::CmdlineIter;
use Cmdline;


#[derive(Debug, Hash, Clone, Eq, PartialEq, PartialOrd)]
pub struct CmdlineSlice<'a>(&'a [u8]);

impl<'a> CmdlineSlice<'a> {
	///Creates cmdline from `&[u8]`.
	#[inline]
	pub fn array(array: &'a [u8]) -> Self {
		CmdlineSlice(array)
	}
	
	#[inline]
	pub fn to_buf(&self) -> CmdlineBuf {
		CmdlineBuf::from(self.0)
	}
}

impl<'a> Cmdline for CmdlineSlice<'a> {
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

/*
impl<'a> Deref for CmdlineSlice<'a> {
	type Target = [u8];
	
	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.array
	}
}
*/
impl<'a> AsRef<[u8]> for CmdlineSlice<'a> {
	#[inline]
	fn as_ref(&self) -> &[u8] {
		self.0
	}
}
impl<'a> AsRef<CmdlineSlice<'a>> for CmdlineSlice<'a> {
	#[inline]
	fn as_ref(&self) -> &CmdlineSlice<'a> {
		self
	}
}


impl<'a> From<&'a [u8]> for CmdlineSlice<'a> {
	#[inline]
	fn from(array: &'a [u8]) -> CmdlineSlice<'a> {
		CmdlineSlice::array(array)
	}
}


