macro_rules! set_c_names {
	($elem:ident, $set:ident, $($field:ident : $var:ident => $s:expr,)*) => {
		impl $elem {
			pub fn from_c_name(c_name: &CStr) -> Option<$elem> {
				use $elem::*;
				let bytes = c_name.to_bytes_with_nul();
				$(
					if bytes == $s {
						return Some($var)
					}
				)*

				None
			}

			pub fn c_name(&self) -> &'static CStr {
				use $elem::*;

				unsafe {
					let name: &'static [u8] = match self {
						$(
							$var => $s,
						)*
					};

					CStr::from_bytes_with_nul_unchecked(name)
				}
			}

			pub fn name(&self) -> &'static str {
				use $elem::*;

				unsafe {
					match self {
						$(
							$var => CStr::from_bytes_with_nul_unchecked($s).to_str().unwrap(),
						)*
					}
				}
			}
		}

		impl fmt::Debug for $elem {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				f.write_str(self.name())
			}
		}

		impl fmt::Display for $elem {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				f.write_str(self.name())
			}
		}

		impl fmt::Debug for $set {
			#[allow(unused_assignments)]
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				write!(f, "[")?;

				let mut first = true;

				$(
					if self.$field {
						if !first { write!(f, ", ")? }
						else { first = false; }
						f.write_str(unsafe {
							CStr::from_bytes_with_nul_unchecked($s).to_str().unwrap()
						})?;
					}
				)*

				write!(f, "]")
			}
		}
	}
}

macro_rules! set_names {
	($elem:ident, $set:ident, $($field:ident : $var:ident => $s:expr,)*) => {
		impl $elem {
			pub fn name(&self) -> &'static str {
				use $elem::*;

				match self {
					$(
						$var => $s
					,)*
				}
			}
		}

		impl fmt::Debug for $elem {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				use $elem::*;
				match self {
					$(
						$var => f.write_str($s)
					,)*
				}
			}
		}

		impl fmt::Display for $elem {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				use $elem::*;
				match self {
					$(
						$var => f.write_str($s)
					,)*
				}
			}
		}

		impl fmt::Debug for $set {
			#[allow(unused_assignments)]
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				write!(f, "[")?;

				let mut first = true;

				$(
					if self.$field {
						if !first { write!(f, ", ")? }
						else { first = false; }
						f.write_str($s)?;
					}
				)*

				write!(f, "]")
			}
		}
	}
}

macro_rules! set {
	(@iter ($this:ident) $i:expr,) => {
		// nothing
	};

	(@iter ($this:ident) $i:expr, $field:ident : $var:ident => $s:expr, $($tfield:ident : $tvar:ident => $ts:expr,)*) => {
		if $this.index == $i {
			$this.index += 1;
			if $this.set.$field {
				return Some($var)
			}
		}

		set!(@iter ($this) $i + 1u8, $($tfield : $tvar => $ts,)*)
	};

	($elem:ident, $set:ident, $iter:ident, $into_iter:ident, $($field:ident : $var:ident => $s:expr,)*) => {
		#[derive(Copy, Clone, PartialEq, Eq)]
		pub enum $elem {
			$(
				$var,
			)*
		}

		/// List of features that are enabled or available.
		#[derive(Clone, PartialEq, Eq)]
		#[allow(missing_docs)]
		pub struct $set {
			$(
				pub $field: bool,
			)*

			pub _unbuildable: Unbuildable,
		}

		impl $set {
			/// Returns an set with all members set to `false`.
			#[inline]
			pub fn none() -> $set {
				$set {
					$($field: false,)*
					_unbuildable: Unbuildable(())
				}
			}

			#[inline]
			pub fn contains(&self, item: $elem) -> bool {
				use $elem::*;
				match item {
					$(
						$var => self.$field,
					)*
				}
			}

			#[inline]
			pub fn insert(&mut self, item: $elem) {
				use $elem::*;
				match item {
					$(
						$var => self.$field = true,
					)*
				}
			}

			#[inline]
			pub fn remove(&mut self, item: $elem) {
				use $elem::*;
				match item {
					$(
						$var => self.$field = false,
					)*
				}
			}

			/// Returns the union of this set and another set.
			#[inline]
			pub fn union(&self, other: &$set) -> $set {
				$set {
					$(
						$field: self.$field || other.$field,
					)*
					_unbuildable: Unbuildable(())
				}
			}

			/// Returns the intersection of this set and another set.
			#[inline]
			pub fn intersection(&self, other: &$set) -> $set {
				$set {
					$(
						$field: self.$field && other.$field,
					)*
					_unbuildable: Unbuildable(())
				}
			}

			/// Returns the difference of another set from this set.
			#[inline]
			pub fn difference(&self, other: &$set) -> $set {
				$set {
					$(
						$field: self.$field && !other.$field,
					)*
					_unbuildable: Unbuildable(())
				}
			}
		}

		pub struct $iter<'a> {
			set: &'a $set,
			index: u8
		}

		impl<'a> Iterator for $iter<'a> {
			type Item = $elem;

			fn next(&mut self) -> Option<$elem> {
				use $elem::*;

				set!(@iter (self) 0u8, $($field : $var => $s,)*);

				None
			}
		}

		pub struct $into_iter {
			set: $set
		}

		impl Iterator for $into_iter {
			type Item = $elem;

			fn next(&mut self) -> Option<$elem> {
				use $elem::*;

				$(
					if self.set.$field {
						self.set.$field = false;
						return Some($var)
					}
				)*

				None
			}
		}

		impl std::iter::IntoIterator for $set {
			type IntoIter = $into_iter;
			type Item = $elem;

			fn into_iter(self) -> $into_iter {
				$into_iter {
					set: self
				}
			}
		}

		impl<'a> std::iter::IntoIterator for &'a $set {
			type IntoIter = $iter<'a>;
			type Item = $elem;

			fn into_iter(self) -> $iter<'a> {
				$iter {
					set: self,
					index: 0
				}
			}
		}
	};
}

macro_rules! extensions {
	($($field:ident : $var:ident => $s:expr,)*) => {
		set!(Extension, Extensions, ExtensionsIter, ExtensionsIntoIter, $($field : $var => $s,)*);
		set_c_names!(Extension, Extensions, $($field : $var => $s,)*);
		impl Copy for Extensions { }
	}
}

macro_rules! validation_layers {
	($($field:ident : $var:ident => $s:expr,)*) => {
		set!(ValidationLayer, ValidationLayers, ValidationLayersIter, ValidationLayersIntoIter, $($field : $var => $s,)*);
		set_c_names!(ValidationLayer, ValidationLayers, $($field : $var => $s,)*);
		impl Copy for ValidationLayers { }
	}
}

macro_rules! features {
	($ffi_ty:path, $tvalue:expr, $($field:ident : $var:ident => $ffi_field:ident : $s:expr,)*) => {
		set!(Feature, Features, FeaturesIter, FeaturesIntoIter, $($field : $var => $s,)*);
		set_names!(Feature, Features, $($field : $var => $s,)*);

		pub(crate) trait IntoFFiFeatures {
			fn into_ffi(self) -> $ffi_ty;
		}

		impl<I: IntoIterator<Item=Feature>> IntoFFiFeatures for I {
			fn into_ffi(self) -> $ffi_ty {
				use Feature::*;

				let mut ffi_set: $ffi_ty = Default::default();

				for feature in self {
					match feature {
						$(
							$var => ffi_set.$ffi_field = $tvalue,
						)*
					}
				}

				ffi_set
			}
		}

		impl From<$ffi_ty> for Features {
			fn from(ffi_set: $ffi_ty) -> Features {
				Features {
					$(
						$field: ffi_set.$ffi_field == $tvalue,
					)*
					_unbuildable: Unbuildable(())
				}
			}
		}
	}
}
