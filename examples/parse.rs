

extern crate cluproccmdline;
use cluproccmdline::array_slice;
use cluproccmdline::Cmdline;


fn main() {
	let mut cmdline = array_slice(b"BOOT_IMAGE=/boot/vmlinuz-linux-zen rw quiet");
	
	{
		//default iter
		let mut iter = cmdline.iter();
		
		assert_eq!(iter.next(), Some((Some(&b"BOOT_IMAGE"[..]), &b"/boot/vmlinuz-linux-zen"[..])));
		assert_eq!(iter.next(), Some((None, &b"rw"[..])));
		assert_eq!(iter.next(), Some((None, &b"quiet"[..])));
		assert_eq!(iter.next(), None);
	}
	
	{
		//one iter
		let mut iter = cmdline.iter_one();
		assert_eq!(iter.next(), Some(&b"rw"[..]));
		assert_eq!(iter.next(), Some(&b"quiet"[..]));
		assert_eq!(iter.next(), None);
	}
	
	{
		//two iter
		let mut iter = cmdline.iter_two();
		assert_eq!(iter.next(), Some((&b"BOOT_IMAGE"[..], &b"/boot/vmlinuz-linux-zen"[..])) );
		assert_eq!(iter.next(), None);
	}
	
}
