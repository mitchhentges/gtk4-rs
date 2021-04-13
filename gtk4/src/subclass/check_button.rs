// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

use super::widget::WidgetImpl;
use crate::{CheckButton, Widget};

pub trait CheckButtonImpl: CheckButtonImplExt + WidgetImpl {
    fn toggled(&self, check_button: &Self::Type) {
        self.parent_toggled(check_button)
    }

    #[cfg(any(feature = "v4_2", feature = "dox"))]
    fn activate(&self, check_button: &Self::Type) {
        self.parent_activate(check_button)
    }
}

pub trait CheckButtonImplExt: ObjectSubclass {
    fn parent_toggled(&self, check_button: &Self::Type);
    #[cfg(any(feature = "v4_2", feature = "dox"))]
    fn parent_activate(&self, check_button: &Self::Type);
}

impl<T: CheckButtonImpl> CheckButtonImplExt for T {
    fn parent_toggled(&self, check_button: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCheckButtonClass;
            if let Some(f) = (*parent_class).toggled {
                f(check_button
                    .unsafe_cast_ref::<CheckButton>()
                    .to_glib_none()
                    .0)
            }
        }
    }

    #[cfg(any(feature = "v4_2", feature = "dox"))]
    fn parent_activate(&self, check_button: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCheckButtonClass;
            if let Some(f) = (*parent_class).activate {
                f(check_button
                    .unsafe_cast_ref::<CheckButton>()
                    .to_glib_none()
                    .0)
            }
        }
    }
}

unsafe impl<T: CheckButtonImpl> IsSubclassable<T> for CheckButton {
    fn class_init(class: &mut glib::Class<Self>) {
        <Widget as IsSubclassable<T>>::class_init(class);

        let klass = class.as_mut();
        klass.toggled = Some(check_button_toggled::<T>);

        #[cfg(any(feature = "v4_2", feature = "dox"))]
        {
            klass.activate = Some(check_button_activate::<T>);
        };
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <Widget as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn check_button_toggled<T: CheckButtonImpl>(ptr: *mut ffi::GtkCheckButton) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<CheckButton> = from_glib_borrow(ptr);

    imp.toggled(wrap.unsafe_cast_ref())
}

#[cfg(any(feature = "v4_2", feature = "dox"))]
unsafe extern "C" fn check_button_activate<T: CheckButtonImpl>(ptr: *mut ffi::GtkCheckButton) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<CheckButton> = from_glib_borrow(ptr);

    imp.activate(wrap.unsafe_cast_ref())
}
