macro_rules! flag_set {
	($(#[$doc:meta])* $item:ident { $($(#[$variant_doc:meta])* $name:ident ($variant:ident) : $flag:path),* } $(#[$set_doc:meta])* $id:ident : $mode:tt $native:ty [$raw:ident]) => {
		$(#[$set_doc])*
		#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
		pub struct $id {
			$(
				$name: bool
			),*
		}

		impl $id {
			flag_set!(@from [$mode] $native { $($name $variant : $flag),* });

			pub fn iter(&self) -> Iter {
				Iter(*self)
			}
		}

		$(#[$doc])*
		#[derive(Clone, Copy, PartialEq, Eq, Debug)]
		#[repr($raw)]
		pub enum $item {
			$(
				$(#[$variant_doc])*
				$variant = $flag.as_raw()
			),*
		}

		impl $item {
			pub fn into_vulkan(self) -> $native {
				<$native>::from_raw(self as $raw)
			}
		}

		pub struct Iter($id);

		impl Iterator for Iter {
			type Item = $item;

			fn next(&mut self) -> Option<$item> {
				$(
					if self.0.$name {
						self.0.$name = false;
						return Some($item::$variant)
					}
				)*

				None
			}
		}
	};
	(@from [flags] $native:ty { $($name:ident $variant:ident : $flag:path),* }) => {
		pub(crate) fn from_vulkan(flags: $native) -> Self {
			Self {
				$(
					$name: flags.contains($flag)
				),*
			}	
		}
	};
	(@from [vec] $native:ty { $($name:ident $variant:ident : $flag:path),* }) => {
		pub(crate) fn from_vulkan(items: Vec<$native>) -> Self {
			let mut v = Self::default();

			for e in items {
				match e {
					$(
						$flag => v.$name = true,
					)*
					_ => ()
				}
			}

			v
		}
	};
}