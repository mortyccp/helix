#[macro_export]
macro_rules! codegen_coercions {
    ({
        type: class,
        rust_name: $rust_name:tt,
        ruby_name: $ruby_name:tt,
        meta: { pub: $pub:tt, reopen: $reopen:tt },
        struct: (),
        methods: $methods:tt
    }) => (
        impl $crate::FromRuby for $rust_name {
            type Checked = $crate::CheckedValue<$rust_name>;

            fn from_ruby(value: $crate::sys::VALUE) -> $crate::CheckResult<$crate::CheckedValue<$rust_name>> {
                use $crate::{CheckedValue, sys};
                use ::std::ffi::{CStr};

                if unsafe { $rust_name == $crate::as_usize(sys::rb_obj_class(value)) } {
                    Ok(unsafe { CheckedValue::new(value) })
                } else {
                    let val = unsafe { CStr::from_ptr(sys::rb_obj_classname(value)).to_string_lossy() };
                    type_error!(format!("No implicit conversion of {} into {}", val, stringify!($rust_name)))
                }
            }

            fn from_checked(checked: $crate::CheckedValue<$rust_name>) -> $rust_name {
                $rust_name { helix: checked.to_value() }
            }
        }

        impl_to_ruby!(&'a $rust_name);
        impl_to_ruby!(&'a mut $rust_name);
    );

    ({
        type: class,
        rust_name: $rust_name:tt,
        ruby_name: $ruby_name:tt,
        meta: { pub: $pub:tt, reopen: false },
        struct: $struct:tt,
        methods: $methods:tt
    }) => (
        impl_struct_to_rust!(&'a $rust_name, $rust_name);
        impl_struct_to_rust!(&'a mut $rust_name, $rust_name);

        impl $crate::ToRuby for $rust_name {
            fn to_ruby(self) -> $crate::ToRubyResult {
                Ok($rust_name::__alloc_with__(Some(Box::new(self))))
            }
        }

        impl_to_ruby!(&'a $rust_name);
        impl_to_ruby!(&'a mut $rust_name);
    );
}


#[doc(hidden)]
#[macro_export]
macro_rules! impl_to_ruby {
    ($rust_name:ty) => {
        item! {
            impl<'a> $crate::ToRuby for $rust_name {
                fn to_ruby(self) -> $crate::ToRubyResult {
                    Ok(self.helix)
                }
            }
        }
    }
}

#[macro_export]
macro_rules! impl_struct_to_rust {
    ($rust_name:ty, $helix_id:tt) => {
        impl<'a> $crate::FromRuby for $rust_name {
            type Checked = $crate::CheckedValue<$rust_name>;

            fn from_ruby(value: $crate::sys::VALUE) -> $crate::CheckResult<$crate::CheckedValue<$rust_name>> {
                use $crate::{CheckedValue, sys};
                use ::std::ffi::{CStr};

                if unsafe { $helix_id == $crate::as_usize(sys::rb_obj_class(value)) } {
                    if unsafe { $crate::sys::Data_Get_Struct_Value(value) == ::std::ptr::null_mut() } {
                        type_error!(format!("Uninitialized {}", $crate::inspect(unsafe { sys::rb_obj_class(value) })))
                    } else {
                        Ok(unsafe { CheckedValue::new(value) })
                    }
                } else {
                    let val = unsafe { CStr::from_ptr(sys::rb_obj_classname(value)).to_string_lossy() };
                    type_error!(format!("No implicit conversion of {} into {}", val, $crate::inspect(unsafe { sys::rb_obj_class(value) })))
                }
            }

            fn from_checked(checked: $crate::CheckedValue<$rust_name>) -> $rust_name {
                unsafe { ::std::mem::transmute($crate::sys::Data_Get_Struct_Value(checked.to_value())) }
            }
        }
    }
}
